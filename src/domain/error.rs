use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{}", _0)]
    NotFound(String),

    #[error("{}", _0)]
    BadRequest(String),

    #[error("{}", _0)]
    InternalServerError(String),
}

impl From<tokio_postgres::Error> for DomainError {
    fn from(err: tokio_postgres::Error) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::PoolError> for DomainError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::BuildError> for DomainError {
    fn from(err: deadpool_postgres::BuildError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::CreatePoolError> for DomainError {
    fn from(err: deadpool_postgres::CreatePoolError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<redis::RedisError> for DomainError {
    fn from(err: redis::RedisError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<validator::ValidationErrors> for DomainError {
    fn from(value: validator::ValidationErrors) -> Self {
        DomainError::BadRequest(value.to_string())
    }
}

impl From<lapin::Error> for DomainError {
    fn from(value: lapin::Error) -> Self {
        DomainError::InternalServerError(value.to_string())
    }
}