use actix_web::{
    put,
    web::{self, Data},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::categories::dto::{self, ResponseCategory},
        utils::response::ApiResponse,
    },
    domain::{categories, error::DomainError},
};

#[utoipa::path(
    put,
    operation_id = "update_categories",
    path = "/categories/{category_id}",
    tag = "categories",
    params(
        ("category_id" = Uuid, Path, description = "Category uuid"),
    ),
    request_body = RequestUpdateCategory,
    responses(
         (status = 200, description = "Category updated",  body = ApiResponseCategory),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
         (status = 404, description = "Category not found",  body = ErrorResponse),
    ),
 )]
#[put("/categories/{category_id}")]
async fn handler(
    state: Data<AppState>,
    param: web::Path<Uuid>,
    body: web::Json<dto::RequestUpdateCategory>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let category = categories::resources::update_by_id::execute(
        state.category_repository.clone(),
        param.to_owned(),
        body.0.into(),
    )
    .await?;

    let response = ApiResponse::<ResponseCategory>::new(vec![category.into()], None, None, None);

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use crate::{
        api::{
            resources::categories::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::categories::{model::CategoryCreateModel, repository::CategoryRepository},
    };

    #[actix_web::test]
    async fn it_should_return_category_updated() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let category_model = CategoryCreateModel::mock_default();
        repositories
            .category_repository
            .insert(&category_model.clone())
            .await
            .unwrap();

        let mock_request_update_category =
            dto::RequestUpdateCategory::mock_default().with_name("Burgers Supreme");
        let req = test::TestRequest::put()
            .uri(&format!("/categories/{}", category_model.id))
            .set_json(mock_request_update_category.clone())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let mock_response_category_updated: ApiResponse<dto::ResponseCategory> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(
            mock_response_category_updated.records.first().unwrap().name,
            mock_request_update_category.name
        )
    }

    #[actix_web::test]
    async fn it_should_return_not_found_error_when_updated_because_invalid_id() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::put()
            .uri(&format!("/categories/{}", Uuid::new_v4()))
            .set_json(dto::RequestUpdateCategory::mock_default().with_name("weapons update 3"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NOT_FOUND);
    }
}
