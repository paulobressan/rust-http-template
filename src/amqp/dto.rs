use serde::Deserialize;

use crate::domain::categories::model::CategoryCreateModel;

#[derive(Debug, Deserialize)]
pub struct CategoryMessage {
    pub name: String,
    pub description: Option<String>,
}

impl From<CategoryMessage> for CategoryCreateModel {
    fn from(value: CategoryMessage) -> Self {
        CategoryCreateModel::new(value.name, value.description)
    }
}
