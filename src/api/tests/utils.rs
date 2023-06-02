use std::sync::Arc;

use crate::{
    api::{config, error::ErrorResponse, lib::AppState, middleware},
    repository::{
        health::PgHealthRepository,
        postgres::{self, init_to_tests},
        redis,
    },
};

use tokio::sync::OnceCell;

use actix_http::Request;

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    error::InternalError,
    test,
    web::{Data, ServiceConfig},
    App, Error, HttpResponse,
};

static INIT_DB: OnceCell<()> = OnceCell::const_new();

async fn setup() {
    INIT_DB
        .get_or_init(|| async {
            dotenv::from_filename(".env.test").ok();

            init_to_tests()
                .await
                .expect("Error to init database to tests");
        })
        .await;
}

pub struct Repositories {
    pub health_repository: Arc<PgHealthRepository>,
}

impl Repositories {
    #[allow(clippy::too_many_arguments)]
    pub fn new(health_repository: Arc<PgHealthRepository>) -> Self {
        Self { health_repository }
    }
}

impl AppState {
    fn mock_default(repositories: &Repositories) -> Data<Self> {
        Data::new(Self {
            health_repository: repositories.health_repository.clone(),
        })
    }
}

pub async fn get_app<F>(
    routes: F,
) -> (
    Repositories,
    impl Service<Request, Response = ServiceResponse<impl MessageBody>, Error = Error>,
)
where
    F: FnOnce(&mut ServiceConfig),
{
    setup().await;

    let json_config = actix_web::web::JsonConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let query_config = actix_web::web::QueryConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let path_config = actix_web::web::PathConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let pool = Arc::new(postgres::init().unwrap());
    let redis_client = Arc::new(redis::init());

    let health_repository = Arc::new(PgHealthRepository::new(pool.clone(), redis_client.clone()));

    let repositories = Repositories::new(health_repository);

    let app_state = AppState::mock_default(&repositories);

    (
        repositories,
        test::init_service(
            App::new()
                .wrap(middleware::cors::default())
                .app_data(json_config.to_owned())
                .app_data(query_config.to_owned())
                .app_data(path_config.to_owned())
                .app_data(app_state)
                .configure(routes),
        )
        .await,
    )
}
