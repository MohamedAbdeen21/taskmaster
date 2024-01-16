use anyhow::Error;
use pyo3::{prelude::*, types::IntoPyDict, types::PyDict};

#[pyclass(unsendable)]
#[derive(Clone, Debug)]
pub struct Task {
    #[pyo3(get)]
    pub name: String,
    pub children: Vec<String>,
    pub parents: Vec<String>,
    callable: PyObject,
}

#[pymethods]
impl Task {
    #[new]
    pub fn new(name: &str, callable: PyObject) -> Result<Self, Error> {
        Ok(Task {
            name: name.to_string(),
            children: Vec::new(),
            parents: Vec::new(),
            callable,
        })
    }

    pub fn __rshift__(&mut self, other: &mut Task) {
        other.parents.push(self.name.clone());
        self.children.push(other.name.clone());
    }

    pub fn info(&self) {
        println!("{:?}", self);
    }

    pub fn execute(&self, caller: &str, args: Option<Py<PyDict>>) -> PyResult<Py<PyDict>> {
        Python::with_gil(|py| -> PyResult<Py<PyDict>> {
            let args = [(caller, args)].into_py_dict(py);
            // args.unwrap_or([(py.None(), py.None())].into_py_dict(py).into_py(py)),
            // );
            let r = self.callable.call(py, (), Some(args))?;
            println!("Executed {} and got {:?}", self.name, r.to_string());
            return Ok(r.extract(py)?);
        })
    }
}
