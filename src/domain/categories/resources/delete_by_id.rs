use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{categories::repository::CategoryRepository, error::DomainError};

pub async fn execute(
    category_repository: Arc<dyn CategoryRepository>,
    category_id: Uuid,
) -> Result<(), DomainError> {
    let has_category = category_repository.find_by_id(&category_id).await?;
    if has_category.is_none() {
        return Err(DomainError::NotFound(String::from("Category id not found")));
    }

    category_repository.delete_by_id(&category_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

    use crate::domain::categories::model::{
        CategoryCreateModel, CategoryModel, CategoryUpdateModel,
    };

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
    async fn it_should_return_void_category_deleted() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(Some(CategoryModel::mock_default())));

        category_repository
            .expect_delete_by_id()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(category_repository), Uuid::new_v4()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_category_not_found() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_find_by_id()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(category_repository), Uuid::new_v4()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
