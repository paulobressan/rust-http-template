use validator::ValidationError;

use std::str::FromStr;

use actix_web::HttpRequest;
use uuid::Uuid;

use crate::domain::error::DomainError;

use crate::api::config;

pub fn validate_page_size_max(page_size: u32) -> Result<(), ValidationError> {
    if page_size > config::get_config().page_size_max {
        return Err(ValidationError::new("page_size greater than the max"));
    }
    Ok(())
}

pub fn get_user_id_by_headers(
    req: &HttpRequest,
    required: bool,
) -> Result<Option<Uuid>, DomainError> {
    let user_id = req.headers().get("x-user-id").map(|value| {
        value
            .to_str()
            .map_err(|_| DomainError::BadRequest("Invalid value in header".to_owned()))
    });

    if required && user_id.is_none() {
        return Err(DomainError::BadRequest(
            "Uuid is required in header".to_owned(),
        ));
    }

    if let Some(user_id) = user_id {
        return Ok(Some(Uuid::from_str(user_id?).map_err(|_| {
            DomainError::BadRequest("Invalid uuid in header".to_owned())
        })?));
    }

    Ok(None)
}
