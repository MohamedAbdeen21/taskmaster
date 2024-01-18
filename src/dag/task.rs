use std::collections::HashMap;

use anyhow::Error;
use itertools::Itertools;
use pyo3::{prelude::*, types::IntoPyDict, types::PyDict};

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
    #[error("Waiting for all parents to finish")]
    NotYet,
    #[error("Function execution failed")]
    Fail(String),
}

#[pyclass]
#[derive(Clone)]
pub struct Task {
    #[pyo3(get)]
    pub name: String,
    callable: PyObject,
    inputs: HashMap<String, Option<Py<PyDict>>>,
    triggers: usize,
}

#[pymethods]
impl Task {
    #[new]
    pub fn new(name: &str, callable: PyObject) -> Result<Self, Error> {
        Ok(Task {
            name: name.to_string(),
            inputs: HashMap::new(),
            triggers: 0,
            callable,
        })
    }
}

impl Task {
    pub fn add_parent(&mut self, parent: &Task) {
        self.inputs.insert(parent.name.clone(), None);
    }

    pub fn execute(
        &mut self,
        caller: &str,
        args: Option<Py<PyDict>>,
    ) -> Result<Py<PyDict>, ExecutionError> {
        self.inputs.insert(caller.to_string(), args.clone());
        self.triggers += 1;

        if self.triggers < self.inputs.len() {
            return Err(ExecutionError::NotYet);
        }

        self.triggers = 0;

        match Python::with_gil(|py| -> PyResult<Py<PyDict>> {
            let args = self.inputs.iter().collect_vec().into_py_dict(py);

            let r = self.callable.call(py, (), Some(args))?;
            println!("Executed {} got {:?}", self.name, r.to_string());
            r.extract(py)
        }) {
            Ok(v) => Ok(v),
            Err(e) => Err(ExecutionError::Fail(e.to_string())),
        }
    }
}
