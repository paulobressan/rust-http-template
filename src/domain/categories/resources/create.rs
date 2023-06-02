use std::sync::Arc;

use crate::domain::categories::model::CategoryModel;
use crate::domain::{
    categories::{model::CategoryCreateModel, repository::CategoryRepository},
    error::DomainError,
};

pub async fn execute(
    category_repository: Arc<dyn CategoryRepository>,
    category_create_model: CategoryCreateModel,
) -> Result<CategoryModel, DomainError> {
    let category = category_repository.insert(&category_create_model).await?;
    Ok(category)
}

#[cfg(test)]
mod tests {
    use crate::domain::categories::model::CategoryUpdateModel;

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    use uuid::Uuid;

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
    async fn it_should_return_category_created() {
        let mut category_repository = MockFakeCategoryRepository::new();

        category_repository
            .expect_insert()
            .return_once(|_| Ok(CategoryModel::mock_default()));

        let result = execute(
            Arc::new(category_repository),
            CategoryCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
