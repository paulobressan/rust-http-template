use std::sync::Arc;

use crate::domain::{
    categories::{model::CategoryModel, repository::CategoryRepository},
    error::DomainError,
};

pub async fn execute(
    category_repository: Arc<dyn CategoryRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<CategoryModel>, u32)>, DomainError> {
    let categories = category_repository.find(&name, &page, &page_size).await?;

    if categories.is_some() {
        return Ok(categories);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

    use crate::domain::categories::model::{CategoryCreateModel, CategoryUpdateModel};

    mock! {
        pub FakeCategoryRepository { }

        #[async_trait]
        impl CategoryRepository for FakeCategoryRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<CategoryModel>, u32)>, DomainError>;
            async fn find_by_id(&self, id: &Uuid) -> Result<Option<CategoryModel>, DomainError>;
            async fn insert(&self,category_create_model: &CategoryCreateModel) -> Result<CategoryModel, DomainError>;
            async fn update_by_id(&self,id: &Uuid,category_update_model: &CategoryUpdateModel) -> Result<CategoryModel, DomainError>;
            async fn delete_by_id(&self, id: &Uuid) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_categories_finded() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![CategoryModel::mock_default()], 1))));

        let (categories, count) = execute(Arc::new(category_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!categories.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut category_repository = MockFakeCategoryRepository::new();
        category_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(category_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
