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
    doc.info.title = String::from("API Template");
    doc.info.description = Some(String::from("Rust API Template using PostgreSQL, Redis, and RabbitMQ. The following template provides a basic structure for developing a Rust API, utilizing the powerful combination of PostgreSQL as a database, Redis as a caching system, and RabbitMQ for asynchronous communication. These technologies offer a comprehensive set of features that can be leveraged to build an efficient and scalable API in Rust."));

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
