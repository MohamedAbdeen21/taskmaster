use anyhow::{anyhow, Result};
use itertools::Itertools;
use pyo3::{prelude::*, types::IntoPyDict, types::PyDict};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub inputs: HashMap<String, Option<Py<PyDict>>>,
    callable: PyObject,
}

impl Task {
    pub fn new(callable: &PyAny) -> Result<Self> {
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
            callable: callable.extract()?,
        })
    }

    pub fn add_parent(&mut self, parent: &Task) {
        self.inputs.insert(parent.name.clone(), None);
    }

    pub fn add_input(&mut self, name: &str, value: Option<Py<PyDict>>) {
        self.inputs.insert(name.to_string(), value);
    }

    pub fn execute(&mut self) -> Result<Option<Py<PyDict>>> {
        Ok(Python::with_gil(|py| -> PyResult<Option<Py<PyDict>>> {
            let args = self.inputs.iter().collect_vec().into_py_dict(py);
            self.callable.call(py, (), Some(args))?.extract(py)
        })?)
    }
}
