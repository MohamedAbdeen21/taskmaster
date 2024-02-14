use super::{Status, Store, DB, SERVER_ADDR};
use anyhow::Result;
use chrono::Utc;
use futures::{future, prelude::*};
use rusqlite::{params, Connection};
use std::{collections::HashMap, sync::Arc};
use tarpc::{
    context, serde_transport,
    server::{self, Channel},
    tokio_serde::formats::Json,
};
use tokio::sync::{Mutex, RwLock};

type SharedHashMap = RwLock<HashMap<String, Mutex<(String, String)>>>;
type SharedConnection = Mutex<Connection>;

#[derive(Clone)]
struct StoreServer {
    db: Arc<SharedConnection>,
    cache: Arc<SharedHashMap>,
}

impl StoreServer {
    fn new(db: Arc<SharedConnection>, cache: Arc<SharedHashMap>) -> Self {
        StoreServer { db, cache }
    }
}

impl Store for StoreServer {
    async fn insert_log(self, _: context::Context, time: String, name: String) -> u64 {
        let conn = self.db.lock().await;
        let insert_query = include_str!("./db/insert.sql");
        conn.query_row(
            insert_query,
            params![time, time, name, Status::Failed],
            |r| r.get(0),
        )
        .unwrap()
    }

    async fn update_log(self, _: context::Context, id: u64, status: Status) {
        let now = Utc::now().naive_utc().to_string();
        let conn = self.db.lock().await;
        let update_query = include_str!("./db/update.sql");
        conn.execute(update_query, params![status, now, id])
            .unwrap();
    }

    async fn upsert_cfg(self, _: context::Context, name: String, time: String, cfg: String) {
        let mut m = self.cache.write().await;
        m.insert(name, Mutex::new((time, cfg)));
    }

    async fn read_cfg(self, _: context::Context, name: String) -> Option<(String, String)> {
        if let Some(cfg) = self.cache.read().await.get(&name) {
            Some(cfg.lock().await.clone())
        } else {
            None
        }
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
pub async fn start() -> Result<()> {
    let mut listener = serde_transport::tcp::listen(&SERVER_ADDR, Json::default).await?;

    let conn = Arc::new(Mutex::new(Connection::open(DB)?));
    let cache = Arc::new(RwLock::new(HashMap::new()));

    {
        let create_query = include_str!("./db/create.sql");
        conn.lock().await.execute(create_query, [])?;
    }

    listener.config_mut().max_frame_length(usize::MAX);
    let _ = listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = StoreServer::new(Arc::clone(&conn), Arc::clone(&cache));
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    Ok(())
}
