use std::collections::HashMap;

use anyhow::{bail, Error};
use pyo3::{prelude::*, types::IntoPyDict, types::PyDict};

#[pyclass]
#[derive(Clone, Debug)]
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

    pub fn add_parent(&mut self, parent: &Task) {
        self.inputs.insert(parent.name.clone(), None);
    }

    pub fn execute(&mut self, caller: &str, args: Option<Py<PyDict>>) -> Result<Py<PyDict>, Error> {
        self.inputs.insert(caller.to_string(), args.clone());
        self.triggers += 1;

        if self.triggers < self.inputs.len() {
            bail!("")
        }

        self.triggers = 0;

        Ok(Python::with_gil(|py| -> PyResult<Py<PyDict>> {
            let args = self
                .inputs
                .iter()
                .map(|(c, a)| (c, a))
                .collect::<Vec<_>>()
                .into_py_dict(py);

            let r = self.callable.call(py, (), Some(args))?;
            println!("Executed {} got {:?}", self.name, r.to_string());
            r.extract(py)
        })?)
    }
}
