use anyhow::Result;
use config::Config;
use sqlx::{Pool, Postgres};

pub mod configuration;
pub mod logger;
mod storage;

pub struct App {
    pool: Pool<Postgres>,
}

impl App {
    pub async fn build(config: &Config) -> Result<Self> {
        tracing::info!("Building application");
        let pool = storage::get_pool(config).await?;
        Ok(Self { pool })
    }
    pub async fn run(&self) -> Result<()> {
        _ = self.pool;
        Ok(())
    }
}
