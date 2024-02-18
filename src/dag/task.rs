use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyCFunction, PyDict, PyTuple},
};
use std::time::Duration;
use std::{collections::HashMap, thread::sleep};

pub type Message = Option<Py<PyAny>>;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Task {
    pub name: String,
    pub deps: HashMap<String, Message>,
    retries: u64,
    retry_delay: f64,
    backoff: f64,
    callable: PyObject,
}

#[pyfunction]
pub fn task(
    retries: Option<u64>,
    retry_delay: Option<f64>,
    backoff: Option<f64>,
    py: Python,
) -> PyResult<&PyCFunction> {
    let f = move |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<Task> {
        let callable: PyObject = args.get_item(0)?.into();
        return Ok(Task {
            deps: HashMap::new(),
            retries: retries.unwrap_or_default(),
            retry_delay: retry_delay.unwrap_or_default(),
            backoff: backoff.unwrap_or_default(),
            name: callable
                .to_string()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_string(),
            callable,
        });
    };
    PyCFunction::new_closure(py, None, None, f)
}

#[pymethods]
impl Task {
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(&self, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<Message> {
        Python::with_gil(|py| -> PyResult<Message> {
            self.callable.call(py, args, kwargs)?.extract(py)
        })
    }
}

impl Task {
    pub fn add_dep(&mut self, parent: &str) {
        self.deps.insert(parent.to_string(), None);
    }

    pub fn set_argument(&mut self, name: &str, value: &Message) {
        self.deps.insert(name.to_string(), value.clone());
    }

    pub fn start(&self, py: Python, args: &PyTuple, kwargs: Message) -> PyResult<Message> {
        let kwargs = kwargs.unwrap_or(self.deps.clone().into_py_dict(py).into());

        let kwargs: Option<&PyDict> = kwargs.extract(py)?;

        let mut msg = self.__call__(args, kwargs);

        if msg.is_ok() {
            return msg;
        }

        for i in 0..self.retries {
            let secs = self.retry_delay + self.retry_delay * self.backoff * i as f64;
            println!("{} failed, sleeping for {}", self.name, secs);
            sleep(Duration::from_secs_f64(secs));

            msg = self.__call__(args, kwargs);
            if msg.is_ok() {
                return msg;
            }
        }

        msg
    }
}
