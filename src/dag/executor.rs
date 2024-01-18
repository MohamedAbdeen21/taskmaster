use std::{collections::HashMap, thread::sleep};

use anyhow::{bail, Error, Result};
use chrono::Duration;
use pyo3::{prelude::*, types::PyDict};

use crate::cron::expression::Expression;

use super::task::{ExecutionError, Task};

#[pyclass]
pub struct Executor {
    pub graph: HashMap<String, Vec<String>>,
    pub tasks: HashMap<String, Task>,
    pub expression: Expression,
    pub roots: Vec<String>,
}

#[pymethods]
impl Executor {
    #[new]
    pub fn new(expression: &str) -> Result<Self, Error> {
        Ok(Executor {
            expression: Expression::from_str(expression)?,
            graph: HashMap::new(),
            roots: Vec::new(),
            tasks: HashMap::new(),
        })
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            // let now = Utc::now().naive_utc();
            // let next = self.expression.next(now);
            // sleep(next.signed_duration_since(now).to_std().unwrap());
            sleep(Duration::seconds(5).to_std().unwrap());
            for root in self.roots.clone().into_iter() {
                self.run("main", root, None)?
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

        self.tasks.insert(rn.clone(), parent);

        Ok(())
    }
}

impl Executor {
    fn run(&mut self, caller: &str, task: String, inputs: Option<Py<PyDict>>) -> Result<()> {
        let t = self.tasks.get_mut(&task).unwrap();
        let output = match t.execute(caller, inputs) {
            Ok(output) => output,
            Err(ExecutionError::NotYet) => return Ok(()),
            Err(e) => bail!(e),
        };

        if let Some(children) = self.graph.clone().get_mut(&task) {
            for child in children {
                self.run(&task, child.clone(), Some(output.clone()))?
            }
        }

        Ok(())
    }
}
