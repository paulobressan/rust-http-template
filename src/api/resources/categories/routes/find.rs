use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        config,
        lib::AppState,
        resources::categories::dto::{self, ResponseCategory},
        utils::response::ApiResponse,
    },
    domain::{categories, error::DomainError},
};

#[utoipa::path(
    get,
    operation_id = "find_categories",
    path = "/categories",
    tag = "categories",
    params(
        dto::RequestFindCategories
    ),
    responses(
         (status = 200, description = "categories",  body = ApiResponseCategory),
         (status = 204, description = "no content categories"),
         (status = 400, description = "Invalid query parameters",  body = ErrorResponse),
    ),
 )]
#[get("/categories")]
async fn handler(
    state: Data<AppState>,
    query: Query<dto::RequestFindCategories>,
) -> Result<HttpResponse, DomainError> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let page_size = query
        .page_size
        .unwrap_or(config::get_config().page_size_default);

    let name = query.name.to_owned();

    let result = categories::resources::find::execute(
        state.category_repository.clone(),
        name,
        page,
        page_size,
    )
    .await?;

    if let Some((categories, count)) = result {
        let response = ApiResponse::<ResponseCategory>::new(
            categories.into_iter().map(|i| i.into()).collect(),
            Some(page),
            Some(count),
            Some(page_size),
        );
        return Ok(HttpResponse::Ok().json(response));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::{
        api::{
            resources::categories::{dto, routes::init_routes},
            tests::utils::get_app,
            utils::response::ApiResponse,
        },
        domain::categories::{model::CategoryCreateModel, repository::CategoryRepository},
    };

    #[actix_web::test]
    async fn it_should_return_categories_finded() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let category_model = CategoryCreateModel::mock_default();
        repositories
            .category_repository
            .insert(&category_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get().uri("/categories").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_categories_finded: ApiResponse<dto::ResponseCategory> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_categories_finded.records.is_empty());
    }
    #[actix_web::test]
    async fn it_should_return_categories_finded_by_query() {
        let (repositories, app) = get_app(init_routes).await;

        //Seed
        let category_model = CategoryCreateModel::mock_default();
        repositories
            .category_repository
            .insert(&category_model.clone())
            .await
            .unwrap();

        let req = test::TestRequest::get()
            .uri(&format!(
                "/categories?name={}&page=1&page_size=24",
                "Burgers",
            ))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());

        let body = test::read_body(res).await;
        let response_categories_finded: ApiResponse<dto::ResponseCategory> =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert!(!response_categories_finded.records.is_empty());
    }

    #[actix_web::test]
    async fn it_should_return_categories_no_content() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/categories?name={}", "no-content",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_error_when_query_parameters_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get()
            .uri(&format!("/categories?page={}&page_size=24", "invalid",))
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
