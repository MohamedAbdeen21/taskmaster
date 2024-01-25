use anyhow::{Context, Result};
use pyo3::{prelude::*, types::PyDict};

// Can't keep state, as PyO3 always passes copies of the
// Python Objects. If we want to cache results to avoid
// reloading the file every single run, we'll have to
// implement this in Python (i.e. in the executor)

#[derive(Debug)]
pub struct ConfigLoader {
    file: Option<String>,
}

impl ConfigLoader {
    pub fn new(file: Option<String>) -> Result<Self> {
        Ok(ConfigLoader { file })
    }

    pub fn load(&self) -> Result<Option<Py<PyDict>>> {
        if self.file.is_none() {
            return Ok(None);
        }

        let file = self.file.clone().unwrap();

        let cfg = Python::with_gil(|py| -> Result<Option<Py<PyDict>>> {
            let locals = PyDict::new(py);
            py.run(
                &format!("import json; s=json.load(open('{}'))", &file),
                None,
                Some(locals),
            )
            .with_context(|| format!("failed to read file {}", file))?;
            let ret: Py<PyDict> = locals.get_item("s").unwrap().extract()?;
            Ok(Some(ret))
        })?;

        Ok(cfg)
    }
}
