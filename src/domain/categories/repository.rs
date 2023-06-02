use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::model::{CategoryCreateModel, CategoryModel, CategoryUpdateModel};

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<CategoryModel>, u32)>, DomainError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<CategoryModel>, DomainError>;
    async fn insert(
        &self,
        category_create_model: &CategoryCreateModel,
    ) -> Result<CategoryModel, DomainError>;
    async fn update_by_id(
        &self,
        id: &Uuid,
        category_update_model: &CategoryUpdateModel,
    ) -> Result<CategoryModel, DomainError>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<(), DomainError>;
}
