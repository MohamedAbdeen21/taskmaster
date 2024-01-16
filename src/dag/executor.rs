use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
};

use anyhow::{anyhow, Error, Result};
use chrono::Duration;
use pyo3::{prelude::*, types::PyDict};

use crate::cron::expression::Expression;

use super::task::Task;

#[pyclass]
pub struct Executor {
    pub graph: HashMap<String, Vec<String>>,
    pub tasks: HashMap<String, Task>,
    pub expression: Expression,
    pub root: String,
}

#[pymethods]
impl Executor {
    #[new]
    pub fn new(expression: &str, root: Task) -> Result<Self, Error> {
        Ok(Executor {
            expression: Expression::from_str(expression)?,
            graph: HashMap::new(),
            tasks: HashMap::new(),
            root: root.name,
        })
    }

    pub fn register_tasks(&mut self, tasks: Vec<Task>) -> Result<()> {
        for task in tasks {
            let name = task.name.clone();
            if self.tasks.insert(name.clone(), task).is_some() {
                return Err(anyhow!("Task {} already exists", name));
            }
        }
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.verify_registred()?;
        loop {
            // let now = Utc::now().naive_utc();
            // let next = self.expression.next(now);
            // sleep(next.signed_duration_since(now).to_std().unwrap());
            sleep(Duration::seconds(5).to_std()?);
            self.execute("main", self.root.clone(), None)
        }
    }
}

impl Executor {
    fn execute(&self, caller: &str, task: String, inputs: Option<Py<PyDict>>) {
        let t = &self.tasks[&task];
        let output = t.execute(caller, inputs).unwrap();
        t.children
            .iter()
            .for_each(|child| self.execute(&task, child.clone(), Some(output.clone())))
    }

    fn verify_registred(&self) -> Result<()> {
        let mut used = self.graph.values().flatten().collect::<HashSet<_>>();
        used.insert(&self.root);
        let registered = self.tasks.keys().collect::<HashSet<&String>>();
        let unregistered: Vec<_> = used.difference(&registered).collect();
        if unregistered.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("Tasks {:?} are unregistered", unregistered))
        }
    }
}
