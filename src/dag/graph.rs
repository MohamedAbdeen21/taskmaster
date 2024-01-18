use super::task::{ExecutionError, Task};
use crate::cron::expression::Expression;
use anyhow::{bail, Error, Result};
use chrono::Duration;
use pyo3::{prelude::*, types::PyDict};
use std::{collections::HashMap, thread::sleep};

#[pyclass]
pub struct Graph {
    graph: HashMap<String, Vec<String>>,
    tasks: HashMap<String, Task>,
    #[allow(dead_code)] // I usually override the cron scheduler while testing
    expression: Expression,
    roots: Vec<String>,
    args: Option<Py<PyDict>>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new(schedule: &str, args: Option<Py<PyDict>>) -> Result<Self, Error> {
        Ok(Graph {
            expression: Expression::from_str(schedule)?,
            graph: HashMap::new(),
            roots: Vec::new(),
            tasks: HashMap::new(),
            args,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            // let now = Utc::now().naive_utc();
            // let next = self.expression.next(now);
            // sleep(next.signed_duration_since(now).to_std()?);
            sleep(Duration::seconds(5).to_std()?);
            for root in self.roots.clone().into_iter() {
                self.run("main", root, self.args.clone())?
            }
        }
    }

    pub fn add_root(&mut self, root: Task) {
        let name = root.name.clone();
        self.roots.push(name.clone());
        self.tasks.insert(name, root);
    }

    pub fn add_edge(&mut self, parent: Task, children: Vec<Task>) -> Result<()> {
        let rn = parent.name.clone();

        self.graph
            .entry(rn.clone())
            .or_default()
            .extend(children.iter().map(|c| c.name.clone()));

        for child in children.into_iter() {
            self.tasks
                .entry(child.name.clone())
                .or_insert(child)
                .add_parent(&parent);
        }

        self.tasks.entry(rn).or_insert(parent);

        Ok(())
    }
}

impl Graph {
    fn run(&mut self, caller: &str, task: String, inputs: Option<Py<PyDict>>) -> Result<()> {
        let t = self.tasks.get_mut(&task).unwrap();
        let output = match t.execute(caller, inputs) {
            Ok(v) => v,
            Err(ExecutionError::NotYet) => return Ok(()),
            Err(e) => bail!(e),
        };

        if let Some(children) = self.graph.clone().get_mut(&task) {
            for child in children {
                self.run(&task, child.clone(), output.clone())?
            }
        }

        Ok(())
    }
}
