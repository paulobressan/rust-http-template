use actix_web::{
    get,
    web::Data,
    HttpResponse,
};

use crate::{
    api::lib::AppState,
    domain::{error::DomainError, health},
};

#[utoipa::path(
    get,
    operation_id = "health",
    path = "/health",
    tag = "health",
    responses(
         (status = 200, description = "health"),
    ),
 )]
#[get("/health")]
async fn handler(state: Data<AppState>) -> Result<HttpResponse, DomainError> {
    let result = health::check::execute(state.health_repository.clone()).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[cfg(test)]
mod tests {
    use crate::api::{resources::health::routes::init_routes, tests::utils::get_app};
    use actix_web::test;

    #[actix_web::test]
    async fn it_should_return_health_check() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }
}
