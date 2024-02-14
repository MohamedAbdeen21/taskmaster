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
    pub fn new(path: String, file: Option<String>) -> Result<Self> {
        if file.is_none() {
            return Ok(ConfigLoader {
                file: None,
                store: None,
            });
        }

        let parent = Path::new(&path).parent().unwrap();
        let file = file.unwrap();
        let cfg_path = Path::new(&file);

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

        if let Some((time, cfg_str)) = self.store.as_ref().unwrap().read_cfg("cfg1".into())? {
            if time == curr_last_mod.to_string() {
                return Python::with_gil(|py| -> Result<Message> {
                    let locals = PyDict::new(py);
                    py.run(
                        &format!("import json; cfg=json.loads('{}')", cfg_str),
                        None,
                        Some(locals),
                    )
                    .with_context(|| format!("Failed to convert cached config: {}", cfg_str))?;
                    let cfg: Py<PyDict> = locals.get_item("cfg").unwrap().extract()?;
                    Ok(Some(cfg))
                });
            }
        }

        let file = self.file.clone().unwrap();

        let (cfg, cfg_str) = Python::with_gil(|py| -> Result<(Message, String)> {
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
        })?;

        self.store.as_ref().unwrap().upsert_cfg(
            "cfg1".into(),
            curr_last_mod.to_string(),
            cfg_str,
        )?;

        Ok(cfg)
    }
}
