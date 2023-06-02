use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::utils::validator::validate_page_size_max,
    domain::categories::model::{CategoryCreateModel, CategoryModel, CategoryUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateCategory {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: Option<String>,
}
impl From<RequestCreateCategory> for CategoryCreateModel {
    fn from(value: RequestCreateCategory) -> Self {
        CategoryCreateModel::new(value.name, value.description)
    }
}
#[cfg(test)]
impl RequestCreateCategory {
    pub fn mock_default() -> Self {
        Self {
            name: "Burgers".to_string(),
            description: Some("The Big Burgers".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RequestUpdateCategory {
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 512))]
    pub description: Option<String>,
}
impl From<RequestUpdateCategory> for CategoryUpdateModel {
    fn from(value: RequestUpdateCategory) -> Self {
        CategoryUpdateModel::new(value.name, value.description)
    }
}
#[cfg(test)]
impl RequestUpdateCategory {
    pub fn mock_default() -> Self {
        Self {
            name: "French fries".to_string(),
            description: Some("The French fries".to_string()),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[derive(Debug, Clone, Deserialize, Validate, IntoParams)]
pub struct RequestFindCategories {
    #[validate(length(max = 64))]
    pub name: Option<String>,
    pub page: Option<u32>,
    #[validate(custom = "validate_page_size_max")]
    pub page_size: Option<u32>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseCategory {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl From<CategoryModel> for ResponseCategory {
    fn from(value: CategoryModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
