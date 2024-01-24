use super::{config_loader::ConfigLoader, task::Task};
use crate::cron::expression::Expression;
use anyhow::{anyhow, Error, Result};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::{prelude::*, types::PyDict};
use std::collections::{HashMap, VecDeque};

#[pyclass]
pub struct Graph {
    graph: HashMap<String, Vec<String>>,
    tasks: HashMap<String, Task>,
    expression: Expression,
    cfg_loader: ConfigLoader,
    #[pyo3(set)]
    execution_order: Vec<String>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new(schedule: &str, config: Option<&str>) -> Result<Self, Error> {
        Ok(Graph {
            expression: Expression::from_str(schedule)?,
            cfg_loader: ConfigLoader::new(config)?,
            graph: HashMap::new(),
            tasks: HashMap::new(),
            execution_order: Vec::new(),
        })
    }

    pub fn add_edge(&mut self, parent: &PyAny, children: Vec<&PyAny>) -> Result<()> {
        let parent = Task::new(parent)?;
        let children: Vec<_> = children.into_iter().map(Task::new).try_collect()?;

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

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn sort(&self) -> Result<Vec<String>> {
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

    pub fn start(&mut self) -> Result<()> {
        let args: Option<Py<PyDict>> = self.cfg_loader.load()?;

        for task_name in self.execution_order.clone().iter() {
            let task = self.tasks.get_mut(task_name).unwrap();

            // root node
            if task.inputs.is_empty() {
                task.add_input("config", args.clone());
            };

            let output = task.execute()?;

            self.graph
                .get(task_name)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|child| {
                    self.tasks
                        .get_mut(child)
                        .unwrap()
                        .add_input(&task_name.clone(), output.clone())
                })
        }

        Ok(())
    }
}
