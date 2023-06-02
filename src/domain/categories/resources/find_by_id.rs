use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    categories::{model::CategoryModel, repository::CategoryRepository},
    error::DomainError,
};

pub async fn execute(
    category_repository: Arc<dyn CategoryRepository>,
    id: Uuid,
) -> Result<Option<CategoryModel>, DomainError> {
    if let Some(category) = category_repository.find_by_id(&id).await? {
        return Ok(Some(category));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::domain::categories::model::{CategoryCreateModel, CategoryUpdateModel};

    use super::*;

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
    async fn it_should_return_category_finded() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(CategoryModel::mock_default())));

        let result = execute(Arc::new(category_repository), Uuid::new_v4()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_category() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(category_repository), Uuid::new_v4()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
