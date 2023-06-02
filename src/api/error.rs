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
            DomainError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse::new(msg)),

            DomainError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse::new(msg))
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
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
