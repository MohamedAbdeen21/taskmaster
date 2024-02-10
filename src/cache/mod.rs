use anyhow::Result;
use bincode::{self, deserialize, serialize};
use redis::{Client, Commands, Connection};
use serde::{de::DeserializeOwned, Serialize};

pub struct Cache {
    db: Connection,
    prefix: String,
}

impl Cache {
    pub fn new(graph: &str, prefix: &str) -> Result<Cache> {
        let db = Client::open("redis://127.0.0.1")?.get_connection()?;
        Ok(Cache {
            db,
            prefix: format!("{}:{}", graph, prefix),
        })
    }

    pub fn get_cache<T: DeserializeOwned>(&mut self, key: &str) -> Result<Option<T>> {
        let full_key = format!("{}:{}", self.prefix, key);
        let result: Option<Vec<u8>> = self.db.get(&full_key)?;

        Ok(result.map(|v| deserialize(&v).unwrap()))
    }

    pub fn push_cache<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        let full_key = format!("{}:{}", self.prefix, key);
        let value = serialize(&value)?;

        self.db.set(&full_key, value)?;

        Ok(())
    }
}
