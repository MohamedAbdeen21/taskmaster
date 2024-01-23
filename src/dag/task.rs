use anyhow::{anyhow, Error};
use itertools::Itertools;
use pyo3::{prelude::*, types::IntoPyDict, types::PyDict};
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
    #[error("Waiting for all parents to finish")]
    NotYet,
    #[error("Task {} execution failed: {}", t, e)]
    Fail { t: String, e: String },
}

#[derive(Clone)]
pub struct Task {
    pub name: String,
    callable: PyObject,
    pub inputs: HashMap<String, Option<Py<PyDict>>>,
    triggered: usize,
}

impl Task {
    pub fn new(callable: &PyAny) -> Result<Self, Error> {
        if !callable.is_callable() {
            return Err(anyhow!("Expected a callable"));
        }

        Ok(Task {
            name: callable
                .to_string()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_string(),
            inputs: HashMap::new(),
            triggered: 0,
            callable: callable.extract()?,
        })
    }

    pub fn add_parent(&mut self, parent: &Task) {
        self.inputs.insert(parent.name.clone(), None);
    }

    pub fn execute(
        &mut self,
        caller: &str,
        args: Option<Py<PyDict>>,
    ) -> Result<Option<Py<PyDict>>, ExecutionError> {
        self.inputs.insert(caller.to_string(), args.clone());
        self.triggered += 1;

        if self.triggered < self.inputs.len() {
            return Err(ExecutionError::NotYet);
        }

        self.triggered = 0;

        Python::with_gil(|py| -> PyResult<Option<Py<PyDict>>> {
            let args = self.inputs.iter().collect_vec().into_py_dict(py);
            match self.callable.call(py, (), Some(args))?.extract(py) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Ok(None),
            }
        })
        .map_err(|e| ExecutionError::Fail {
            t: self.name.clone(),
            e: e.to_string(),
        })
    }
}
