use super::{
    config_loader::ConfigLoader,
    task::{Message, Task},
};
use crate::{cron::expression::Expression, store};
use anyhow::{anyhow, Error, Result};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::types::PyDict;
use pyo3::{prelude::*, types::PyTuple};
use std::collections::{HashMap, VecDeque};

#[pyclass]
pub struct Graph {
    name: String,
    graph: HashMap<String, Vec<String>>,
    tasks: HashMap<String, Task>,
    expression: Option<Expression>,
    cfg_loader: ConfigLoader,
    execution_order: Vec<String>,
    store: store::client::Client,
}

#[pymethods]
impl Graph {
    #[new]
    fn new(name: String, schedule: &str, config: Option<&str>) -> Result<Self, Error> {
        let py_file = if config.is_some() {
            Python::with_gil(|py| -> Result<String> {
                let locals = PyDict::new(py);
                py.run("import os; s=os.path.abspath(__file__)", None, Some(locals))?;
                let ret: String = locals.get_item("s").unwrap().extract()?;
                Ok(ret)
            })?
        } else {
            "/".into()
        };

        let expression = if schedule.to_lowercase() == "manual" {
            None
        } else {
            Some(Expression::from_str(schedule)?)
        };

        Ok(Graph {
            name,
            expression,
            cfg_loader: ConfigLoader::new(py_file, config)?,
            graph: HashMap::new(),
            tasks: HashMap::new(),
            execution_order: Vec::new(),
            store: store::client::Client::new()?,
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
        let mut children = children.unwrap_or_default();

        for parent in parents.iter() {
            self.graph
                .entry(parent.name.clone())
                .or_default()
                .extend(children.iter().map(|c| c.name.clone()));

            for child in children.iter_mut() {
                child.add_dep(&parent.name);
            }
        }

        children.iter().for_each(|child| {
            self.tasks
                .entry(child.name.clone())
                .or_insert(child.clone());
        });

        parents.iter().for_each(|child| {
            self.tasks
                .entry(child.name.clone())
                .or_insert(child.clone());
        });

        Ok(())
    }

    fn next(&self) -> Option<NaiveDateTime> {
        self.expression
            .as_ref()
            .map(|exp| exp.next(Utc::now().naive_utc()))
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn is_manual(&self) -> bool {
        self.expression.is_none()
    }

    fn sort(&self) -> Result<Vec<String>> {
        let mut sorted = vec![];
        let mut in_degrees: HashMap<String, usize> = self
            .tasks
            .values()
            .map(|task| (task.name.clone(), task.deps.len()))
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

    fn name(&self) -> String {
        self.name.clone()
    }

    #[pyo3(signature=(*args, **kwargs))]
    fn __call__(
        &mut self,
        py: Python,
        args: &PyTuple,
        kwargs: Option<Py<PyAny>>,
    ) -> Result<Message> {
        let run_id = self.store.insert_log(self.name.clone())?;
        match self.run(py, args, kwargs) {
            Ok(msg) => {
                self.store.update_log(run_id, store::Status::Completed)?;
                Ok(msg)
            }
            Err(e) => {
                eprintln!("Graph {} failed: {}", &self.name, e);
                self.store.update_log(run_id, store::Status::Failed)?;
                Err(e)
            }
        }
    }

    fn run(&mut self, py: Python, args: &PyTuple, mut kwargs: Message) -> Result<Message> {
        if self.is_empty() || !self.is_sorted() {
            return Err(anyhow!("Please call `commit()` before running the Graph"));
        }

        if kwargs.is_none() {
            kwargs = self.cfg_loader.load()?;
        }

        let mut output = None;

        for task_name in self.execution_order.iter() {
            let task = self.tasks.get_mut(task_name).unwrap();

            if task.deps.is_empty() {
                // root nodes
                output = task.start(py, args, kwargs.clone())?;
            } else {
                // leaf and inner nodes
                output = task.start(py, PyTuple::empty(py), None)?;
            }

            // println!("{:?}", self.execution_order);
            // println!("{:?}", self.tasks);
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

        Ok(output)
    }
}
