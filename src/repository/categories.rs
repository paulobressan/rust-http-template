use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use uuid::Uuid;

use crate::domain::{
    categories::{
        model::{CategoryCreateModel, CategoryModel, CategoryUpdateModel},
        repository::CategoryRepository,
    },
    error::DomainError,
};

const QUERY_FIND_CATEGORY: &str = "
    select
        id as category_id,
        name as category_name,
        description as category_description,
        is_active as category_is_active,
        created_at as category_created_at,
        updated_at as category_updated_at,
        count(*) over ()::OID as count
    from
        category";

const QUERY_FIND_CATEGORY_BY_ID: &str = "
    select
        id as category_id,
        name as category_name,
        description as category_description,
        is_active as category_is_active,
        created_at as category_created_at,
        updated_at as category_updated_at
    from
        category
    where 
        id = $1;";

const QUERY_INSERT_CATEGORY: &str = "
    insert into category
        (id, name, description)
    values
        ($1,$2,$3)
    returning
        id as category_id,
        name as category_name,
        description as category_description,
        is_active as category_is_active,
        created_at as category_created_at,
        updated_at as category_updated_at;";

const QUERY_UPDATE_CATEGORY_BY_ID: &str = "
    update
        category 
    set
        name=$2,
        description=$3,
        updated_at=now()
    where
        id = $1
    returning
        id as category_id,
        name as category_name,
        description as category_description,
        is_active as category_is_active,
        created_at as category_created_at,
        updated_at as category_updated_at;";

const QUERY_DELETE_CATEGORY_BY_ID: &str = "
            delete from
                category 
            where
                id = $1;";

pub struct PgCategoryRepository {
    pool: Arc<Pool>,
}
impl PgCategoryRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PgCategoryRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<CategoryModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "category.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_CATEGORY);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let categories: Vec<CategoryModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((categories, count)));
        }

        return Ok(None);
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<CategoryModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_CATEGORY_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        category_create_model: &CategoryCreateModel,
    ) -> Result<CategoryModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_CATEGORY).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &category_create_model.id,
                    &category_create_model.name,
                    &category_create_model.description,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_id(
        &self,
        id: &Uuid,
        category_update_model: &CategoryUpdateModel,
    ) -> Result<CategoryModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_CATEGORY_BY_ID).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    id,
                    &category_update_model.name,
                    &category_update_model.description,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_CATEGORY_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for CategoryModel {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("category_id"),
            name: row.get("category_name"),
            description: row.get("category_description"),
            is_active: row.get("category_is_active"),
            created_at: row.get("category_created_at"),
            updated_at: row.get("category_updated_at"),
        }
    }
}
