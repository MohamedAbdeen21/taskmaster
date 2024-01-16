use std::fmt::Display;

use anyhow::{Error, Result};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::prelude::*;

use crate::cron::expression::Expression;

#[pyclass(unsendable)]
#[derive(Default)]
pub struct Task {
    #[pyo3(get)]
    pub name: String,
    pub start: bool,
    pub expression: Option<Expression>,
    pub children: Vec<*mut Task>,
    pub parents: Vec<*mut Task>,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            return Ok(());
        }

        if let Some(expression) = &self.expression {
            write!(f, "{} -> {}", expression.fields.join(" "), self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }

        unsafe {
            let c = self
                .children
                .iter()
                .map(|child| (*(*child)).name.clone())
                .join(", ");
            writeln!(f, " -> [{}]", c)?;
            self.children
                .iter()
                .for_each(|c| write!(f, "{}", (*(*c))).unwrap())
        }
        Ok(())
    }
}

#[pymethods]
impl Task {
    #[new]
    pub fn new(name: &str, expression: Option<&str>) -> Result<Self, Error> {
        if let Some(expression) = expression {
            let exp = Expression::from_str(expression)?;
            return Ok(Task {
                name: name.to_string(),
                expression: Some(exp),
                start: true,
                ..Task::default()
            });
        }

        Ok(Task {
            name: name.to_string(),
            start: false,
            ..Task::default()
        })
    }

    pub fn next(&self) -> Option<NaiveDateTime> {
        if let Some(e) = &self.expression {
            Some(e.next(Utc::now().naive_utc()))
        } else {
            None
        }
    }

    pub fn __rshift__(&mut self, other: &mut Task) {
        other.parents.push(self);
        self.children.push(other);
        // println!("{} got rshift on {}", self.name, other.name);
        // unsafe { self.children.iter().for_each(|x| println!("{}", *(*x))) }
    }

    pub fn info(&self) {
        println!("{}", self);
    }

    pub fn execute(&self, who: String) {
        println!("{} executed {}", who, self.name);
        unsafe {
            self.children
                .iter()
                .for_each(|c| (*(*c)).execute(self.name.clone()))
        }
    }
}
