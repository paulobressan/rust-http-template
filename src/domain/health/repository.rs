use async_trait::async_trait;

use crate::domain::error::DomainError;

#[async_trait]
pub trait HealthRepository: Send + Sync {
    async fn get_now(&self) -> Result<String, DomainError>;
    async fn ping(&self) -> Result<String, DomainError>;
}
