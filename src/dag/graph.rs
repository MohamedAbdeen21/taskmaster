use super::{config_loader::ConfigLoader, task::Task};
use crate::cron::expression::Expression;
use anyhow::{anyhow, Error, Result};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, VecDeque};

#[pyclass]
pub struct Graph {
    name: String,
    graph: HashMap<String, Vec<String>>,
    tasks: HashMap<String, Task>,
    expression: Expression,
    cfg_loader: ConfigLoader,
    execution_order: Vec<String>,
}

#[pymethods]
impl Graph {
    #[new]
    fn new(name: &str, schedule: &str, config: Option<String>) -> Result<Self, Error> {
        let py_file = Python::with_gil(|py| -> Result<String> {
            let locals = PyDict::new(py);
            py.run("import os; s=os.path.abspath(__file__)", None, Some(locals))?;
            let ret: String = locals.get_item("s").unwrap().extract()?;
            Ok(ret)
        })?;

        Ok(Graph {
            name: name.to_string(),
            expression: Expression::from_str(schedule)?,
            cfg_loader: ConfigLoader::new(py_file, config)?,
            graph: HashMap::new(),
            tasks: HashMap::new(),
            execution_order: Vec::new(),
        })
    }

    fn commit(&mut self) -> Result<()> {
        if self.is_empty() {
            return Err(anyhow!("Graph is empty"));
        }

        if !self.is_sorted() {
            self.execution_order = self.sort()?;
        }

        Ok(())
    }

    fn add_edges(&mut self, parents: Vec<Task>, children: Option<Vec<Task>>) -> Result<()> {
        let children = children.unwrap_or_default();
        for parent in parents {
            self.graph
                .entry(parent.name.clone())
                .or_default()
                .extend(children.iter().map(|c| c.name.clone()));

            self.tasks.entry(parent.name.clone()).or_insert(parent);
        }

        Ok(())
    }

    fn next(&self) -> NaiveDateTime {
        self.expression.next(Utc::now().naive_utc()).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn sort(&self) -> Result<Vec<String>> {
        let mut sorted = vec![];
        let mut in_degrees: HashMap<String, usize> = self
            .tasks
            .values()
            .map(|task| (task.name.clone(), task.inputs.len()))
            .collect();

        let mut queue: VecDeque<_> = in_degrees
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k)
            .cloned()
            .collect();

        queue.iter().for_each(|task| {
            in_degrees.remove(task);
        });

        loop {
            if queue.is_empty() && in_degrees.is_empty() {
                return Ok(sorted);
            }

            if queue.is_empty() && !in_degrees.is_empty() {
                return Err(anyhow!("Graph has a cycle"));
            }

            let task = queue.pop_front().unwrap();

            sorted.push(task.clone());

            self.graph
                .get(&task)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|child| {
                    in_degrees.entry(child.clone()).and_modify(|v| *v -= 1);
                });

            let next = in_degrees
                .iter()
                .filter(|(_, v)| **v == 0)
                .map(|(k, _)| k)
                .cloned()
                .collect_vec();

            next.iter().for_each(|task| {
                in_degrees.remove(task).unwrap();
            });

            queue.extend(next);
        }
    }

    fn is_sorted(&self) -> bool {
        !self.execution_order.is_empty()
    }

    fn start(&mut self, py: Python) -> Result<()> {
        let args = &self.cfg_loader.load()?;

        if let Some(cfg) = &args {
            println!("{} is running with config {}", &self.name, cfg);
        } else {
            println!("{} is running with no configs", &self.name);
        }

        if self.is_empty() || !self.is_sorted() {
            return Err(anyhow!("Please call `commit()` before running the Graph"));
        }

        for task_name in self.execution_order.iter() {
            let task = self.tasks.get_mut(task_name).unwrap();

            // root node
            if task.inputs.is_empty() && args.is_some() {
                task.set_argument("config", args);
            };

            let output = task.start(py)?;

            self.graph
                .get(task_name)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|child| {
                    self.tasks
                        .get_mut(child)
                        .unwrap()
                        .set_argument(task_name, &output)
                })
        }

        Ok(())
    }
}
