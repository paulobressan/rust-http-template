use std::sync::Arc;

use crate::domain::error::DomainError;

use super::repository::HealthRepository;

pub async fn execute(health_repository: Arc<dyn HealthRepository>) -> Result<String, DomainError> {
    let date_now = health_repository.get_now().await?;
    let redis_pong = health_repository.ping().await?;
    Ok(format!("POSTGRES: {date_now} REDIS: {redis_pong}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        pub FakeRepository { }

        #[async_trait]
        impl HealthRepository for FakeRepository {
            async fn get_now(&self) -> Result<String, DomainError>;
            async fn ping(&self) -> Result<String, DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_date_now_of_db() {
        let mut repository = MockFakeRepository::new();
        repository
            .expect_get_now()
            .once()
            .returning(|| Ok(String::from("2023-01-15 13:05:27.205253+00")));
        repository
            .expect_ping()
            .return_once(|| Ok(String::from("pong")));

        let check = execute(Arc::new(repository)).await.unwrap();

        assert_eq!(check, "POSTGRES: 2023-01-15 13:05:27.205253+00 REDIS: pong");
    }

    #[tokio::test]
    async fn it_should_return_db_error() {
        let mut repository = MockFakeRepository::new();
        repository
            .expect_get_now()
            .once()
            .returning(|| Err(DomainError::InternalServerError("Connect DB".to_string())));

        let res = execute(Arc::new(repository)).await;

        match res {
            Err(DomainError::InternalServerError(_)) => {}
            _ => unreachable!(),
        }
    }
}
