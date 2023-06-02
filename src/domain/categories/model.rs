use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategoryCreateModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
impl CategoryCreateModel {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
        }
    }
}

#[cfg(test)]
impl CategoryCreateModel {
    pub fn mock_default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: "Burgers".to_string(),
            description: Some("The Big Burgers".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryUpdateModel {
    pub name: String,
    pub description: Option<String>,
}
impl CategoryUpdateModel {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
        }
    }
}
#[cfg(test)]
impl CategoryUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "French fries".to_string(),
            description: Some("The French fries".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[cfg(test)]
impl CategoryModel {
    pub fn mock_default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: "Burgers".to_string(),
            description: Some("The Big Burgers".to_string()),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}
