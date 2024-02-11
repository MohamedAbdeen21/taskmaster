use super::task::Message;
use crate::cache::Cache;
use anyhow::{Context, Result};
use pyo3::{prelude::*, types::PyDict};
use std::{fs, path::Path, time::SystemTime};

// Can't keep state as graphs are ran in subprocesses.
// If we want to cache results to avoid
// reloading the file every single run, we'll have to
// implement this in Python (i.e. in the executor)
// or use a state file

#[derive(Debug)]
pub struct ConfigLoader {
    file: Option<String>,
}

impl ConfigLoader {
    pub fn new(path: String, file: Option<String>) -> Result<Self> {
        if file.is_none() {
            return Ok(ConfigLoader { file: None });
        }

        let parent = Path::new(&path).parent().unwrap();
        let file = file.unwrap();
        let cfg_path = Path::new(&file);

        let c = ConfigLoader {
            file: parent.join(cfg_path).to_str().map(|s| s.to_string()),
        };
        c.load()?; // ensure file exists and is readable
        Ok(c)
    }

    pub fn load(&self) -> Result<Message> {
        let db = match Cache::new("test_graph", "cfg") {
            Ok(db) => Some(db),
            _ => None,
        };

        if self.file.is_none() {
            return Ok(None);
        }

        let m = fs::metadata(self.file.clone().unwrap())?;
        let r = m.modified()?;

        if let Some(mut db) = db {
            if let Some(lm) = db.get_cache::<SystemTime>("last_modified")? {
                if lm == r {
                    println!("Configs were cached");
                }
            }

            db.push_cache::<SystemTime>("last_modified", r)?;
        }

        let file = self.file.clone().unwrap();

        let cfg = Python::with_gil(|py| -> Result<Message> {
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

        // TODO: Cache args
        // db.push_cache("cfg", cfg.clone().unwrap().into())?;

        Ok(cfg)
    }
}
