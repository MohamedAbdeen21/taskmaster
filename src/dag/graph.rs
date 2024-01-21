use super::task::{ExecutionError, Task};
use crate::cron::expression::Expression;
use anyhow::{bail, Error, Result};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::{prelude::*, types::PyDict};
use std::collections::HashMap;

#[pyclass]
#[derive(Clone)]
pub struct Graph {
    graph: HashMap<String, Vec<String>>,
    tasks: HashMap<String, Task>,
    pub expression: Expression,
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
            tasks: HashMap::new(),
            roots: Vec::new(),
            args,
        })
    }

    pub fn add_root(&mut self, root: &PyAny) -> Result<()> {
        let root = Task::new(root)?;
        let name = root.name.clone();
        self.roots.push(name.clone());
        self.tasks.insert(name, root);
        Ok(())
    }

    pub fn add_edge(&mut self, parent: &PyAny, children: Vec<&PyAny>) -> Result<()> {
        let parent = Task::new(parent)?;
        let children: Vec<_> = children
            .into_iter()
            .map(|child| Task::new(child))
            .try_collect()?;

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

    pub fn next(&self) -> NaiveDateTime {
        self.expression.next(Utc::now().naive_utc()).unwrap()
    }

    pub fn start(&mut self) -> Result<()> {
        self.roots
            .clone()
            .into_iter()
            .map(|root| self.run("main", root, self.args.clone()))
            .try_collect()?;
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

        self.graph
            .clone()
            .get_mut(&task)
            .unwrap_or(&mut vec![])
            .iter()
            .map(|child| self.run(&task, child.clone(), output.clone()))
            .try_collect()?;

        Ok(())
    }
}
