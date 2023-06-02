use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use crate::domain::{error::DomainError, health::repository::HealthRepository};

pub struct PgHealthRepository {
    pool: Arc<Pool>,
    redis_client: Arc<redis::Client>,
}
impl PgHealthRepository {
    pub fn new(pool: Arc<Pool>, redis_client: Arc<redis::Client>) -> Self {
        Self { pool, redis_client }
    }
}

#[async_trait]
impl HealthRepository for PgHealthRepository {
    async fn get_now(&self) -> Result<String, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare("SELECT NOW()::VARCHAR;").await?;
        let result = client.query_one(&stmt, &[]).await?;
        let response: String = result.get("now");
        Ok(response)
    }

    async fn ping(&self) -> Result<String, DomainError> {
        let mut con = self.redis_client.get_async_connection().await?;
        let pong: String = redis::cmd("PING").query_async(&mut con).await?;
        Ok(pong)
    }
}
