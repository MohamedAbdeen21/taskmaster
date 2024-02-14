use super::{Status, StoreClient, SERVER_ADDR};
use anyhow::Result;
use chrono::Utc;
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::runtime::{Builder, Runtime};

pub struct Client {
    client: StoreClient,
    rt: Runtime, // I HATE async programming
}

impl Client {
    pub fn new() -> Result<Self> {
        let rt = Builder::new_current_thread().enable_all().build()?;

        let client = rt.block_on(async move {
            let mut transport = tarpc::serde_transport::tcp::connect(&SERVER_ADDR, Json::default);
            transport.config_mut().max_frame_length(usize::MAX);
            let transport = transport.await.unwrap();
            StoreClient::new(client::Config::default(), transport).spawn()
        });
        Ok(Client { client, rt })
    }

    pub fn insert_log(&self, graph: String) -> Result<u64> {
        Ok(self.rt.block_on(async move {
            self.client
                .insert_log(
                    context::current(),
                    Utc::now().naive_utc().to_string(),
                    graph,
                )
                .await
        })?)
    }

    pub fn update_log(&self, id: u64, status: Status) -> Result<()> {
        self.rt.block_on(
            async move { self.client.update_log(context::current(), id, status).await },
        )?;
        Ok(())
    }

    pub fn upsert_cfg(&self, name: String, time: String, cfg: String) -> Result<()> {
        self.rt.block_on(async move {
            self.client
                .upsert_cfg(context::current(), name, time, cfg)
                .await
        })?;
        Ok(())
    }

    pub fn read_cfg(&self, name: String) -> Result<Option<(String, String)>> {
        Ok(self
            .rt
            .block_on(async move { self.client.read_cfg(context::current(), name).await })?)
    }
}
