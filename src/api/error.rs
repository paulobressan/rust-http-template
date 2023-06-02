use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::error::DomainError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}
impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}
impl actix_web::error::ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DomainError::PreconditionFailed(msg) => {
                HttpResponse::PreconditionFailed().json(ErrorResponse::new(msg))
            }
            DomainError::Forbidden(msg) => HttpResponse::Forbidden().json(ErrorResponse::new(msg)),
            DomainError::PreconditionRequired(msg) => {
                HttpResponse::PreconditionRequired().json(ErrorResponse::new(msg))
            }
            DomainError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse::new(msg)),
            DomainError::Conflict(msg) => HttpResponse::Conflict().json(ErrorResponse::new(msg)),
            DomainError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse::new(msg))
            }
            DomainError::NotAcceptable(msg) => {
                HttpResponse::NotAcceptable().json(ErrorResponse::new(msg))
            }
            err => {
                log::error!("{}", err);
                HttpResponse::InternalServerError()
                    .json(ErrorResponse::new("Internal Server Error"))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::Forbidden(_) => StatusCode::FORBIDDEN,
            DomainError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            DomainError::PreconditionRequired(_) => StatusCode::PRECONDITION_REQUIRED,
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::Conflict(_) => StatusCode::CONFLICT,
            DomainError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
