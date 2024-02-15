use super::task::Message;
use crate::store;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use pyo3::{prelude::*, types::PyDict};
use std::{fs, path::Path};

pub struct ConfigLoader {
    file: Option<String>,
    store: Option<store::client::Client>,
}

impl ConfigLoader {
    pub fn new(path: String, file: Option<&str>) -> Result<Self> {
        if file.is_none() {
            return Ok(ConfigLoader {
                file: None,
                store: None,
            });
        }

        let parent = Path::new(&path).parent().unwrap();
        let cfg_path = Path::new(file.unwrap());

        let c = ConfigLoader {
            file: parent.join(cfg_path).to_str().map(|s| s.to_string()),
            store: Some(store::client::Client::new()?),
        };
        c.load()?; // ensure file exists and is readable
        Ok(c)
    }

    pub fn load(&self) -> Result<Message> {
        if self.file.is_none() {
            return Ok(None);
        }

        let m = fs::metadata(self.file.clone().unwrap())?.modified()?;
        let curr_last_mod: DateTime<Utc> = m.into();

        let store = self.store.as_ref().unwrap();

        if let Some((time, cfg_str)) = store.read_cfg("cfg1".into())? {
            if time == curr_last_mod.to_string() {
                return self.deserialize(&cfg_str);
            }
        }

        let file = self.file.clone().unwrap();

        let (cfg, cfg_str) = self.read_and_serialize(&file)?;

        store.upsert_cfg("cfg1".into(), curr_last_mod.to_string(), cfg_str)?;

        Ok(cfg)
    }

    fn read_and_serialize(&self, file: &str) -> Result<(Message, String)> {
        Python::with_gil(|py| -> Result<(Message, String)> {
            let locals = PyDict::new(py);
            py.run(
                &format!(
                    "import json; cfg=json.load(open('{}')); s=json.dumps(cfg)",
                    &file
                ),
                None,
                Some(locals),
            )
            .with_context(|| format!("failed to read file {}", file))?;
            let cfg: Py<PyDict> = locals.get_item("cfg").unwrap().extract()?;
            let cfg_str: String = locals.get_item("s").unwrap().extract()?;
            Ok((Some(cfg), cfg_str))
        })
    }

    fn deserialize(&self, s: &str) -> Result<Message> {
        Python::with_gil(|py| -> Result<Message> {
            let locals = PyDict::new(py);
            py.run(
                &format!("import json; cfg=json.loads('{}')", s),
                None,
                Some(locals),
            )
            .with_context(|| format!("Failed to convert cached config: {}", s))?;
            let cfg: Py<PyDict> = locals.get_item("cfg").unwrap().extract()?;
            Ok(Some(cfg))
        })
    }
}
