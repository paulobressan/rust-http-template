use actix_web::{get, http::header, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::api::resources::health::routes::check::handler,),
    components(schemas(crate::api::error::ErrorResponse, crate::api::utils::response::Meta,))
)]
struct ApiDoc;

#[get("/docs")]
async fn redirect() -> impl Responder {
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/docs/"))
        .finish()
}

pub fn swagger() -> SwaggerUi {
    let mut doc = ApiDoc::openapi();
    doc.info.title = String::from("Inventories API");
    doc.info.description = Some(String::from(
        "Application responsible for the inventory of the game and players",
    ));

    SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", doc)
}

#[cfg(test)]
mod tests {

    use crate::api::{middleware, resources::swagger::routes::init_routes};
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_swagger() {
        dotenv::from_filename(".env.test").ok();

        let app = test::init_service(
            App::new()
                .wrap(middleware::cors::default())
                .configure(init_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/docs/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/docs").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }
}
