// use std::collections::{HashMap, HashSet};

// use anyhow::{Error, Result};
// use pyo3::prelude::*;

// use crate::cron::expression::Expression;

// use super::task::Task;

// #[pyclass]
// pub struct Engine {
//     pub dag: HashMap<String, Vec<String>>,
//     #[pyo3(get)]
//     pub tasks: HashSet<Task>,
//     pub cron: Expression,
//     pub root: String,
// }

// #[pymethods]
// impl Engine {
//     #[new]
//     pub fn new(expression: &str) -> Result<Self, Error> {
//         return Ok(Engine {
//             cron: Expression::from_str(expression)?,
//             dag: HashMap::new(),
//             tasks: HashSet::new(),
//             root: String::new(),
//         });
//     }
// }
