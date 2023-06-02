use validator::ValidationError;

use crate::api::config;

pub fn validate_page_size_max(page_size: u32) -> Result<(), ValidationError> {
    if page_size > config::get_config().page_size_max {
        return Err(ValidationError::new("page_size greater than the max"));
    }
    Ok(())
}
