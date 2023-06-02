use actix_web::{
    delete,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    api::lib::AppState,
    domain::{categories, error::DomainError},
};

#[utoipa::path(
    delete,
    operation_id = "delete_categories",
    path = "/categories/{category_id}",
    tag = "categories",
    params(
        ("category_id" = Uuid, Path, description = "category uuid"),
    ),
    responses(
         (status = 204, description = "category deleted"),
         (status = 400, description = "Invalid category id",  body = ErrorResponse),
         (status = 404, description = "category not found",  body = ErrorResponse),
         (status = 409, description = "category is in use",  body = ErrorResponse),
    ),
 )]
#[delete("/categories/{category_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
) -> Result<HttpResponse, DomainError> {
    categories::resources::delete_by_id::execute(
        state.category_repository.clone(),
        param.to_owned(),
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{resources::categories::routes::init_routes, tests::utils::get_app},
        domain::categories::{model::CategoryCreateModel, repository::CategoryRepository},
    };

    #[actix_web::test]
    async fn it_should_return_void_category_deleted() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let category_model = CategoryCreateModel::mock_default();
        repositories
            .category_repository
            .insert(&category_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/categories/{}", category_model.id))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_deleting() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/categories/{}", Uuid::new_v4()))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
