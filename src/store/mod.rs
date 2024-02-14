use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
pub mod client;
pub mod server;

const SERVER_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 9009);
const DB: &str = "./log.db";

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Completed,
    Running,
    Failed,
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Self::Failed => "failed",
            Self::Completed => "completed",
            Self::Running => "running",
        }
        .to_sql()
    }
}

#[tarpc::service]
trait Store {
    async fn insert_log(time: String, name: String) -> u64;
    async fn update_log(id: u64, status: Status);

    async fn read_cfg(name: String) -> Option<(String, String)>;
    async fn upsert_cfg(name: String, time: String, cfg: String);
}
