use actix_web::{
    error::InternalError,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use deadpool_postgres::Pool;
use redis::Client;
use serde_qs::actix::QsQueryConfig;
use std::{error::Error, sync::Arc};

use crate::{
    api::{
        config,
        error::ErrorResponse,
        middleware,
        resources::{health, swagger},
    },
    domain::health::repository::HealthRepository,
    repository::{health::PgHealthRepository, postgres},
};

pub struct AppState {
    pub health_repository: Arc<dyn HealthRepository>,
}

pub async fn run(pg_pool: Arc<Pool>, redis_client: Arc<Client>) -> Result<(), Box<dyn Error>> {
    postgres::run_migrations().await?;

    let json_config = web::JsonConfig::default().error_handler(|err, _| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let query_config = web::QueryConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let path_config = web::PathConfig::default().error_handler(|err, _req| {
        let http_error =
            HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
        InternalError::from_response(err, http_error).into()
    });

    let repositories = Data::new(AppState {
        health_repository: Arc::new(PgHealthRepository::new(
            pg_pool.clone(),
            redis_client.clone(),
        )),
    });

    let web_addr = &config::get_config().web_addr;
    println!("server listener in: {web_addr}");

    HttpServer::new(move || {
        let qs_config = QsQueryConfig::default()
            .error_handler(|err, _| {
                let http_error =
                    HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string().as_str()));
                InternalError::from_response(err, http_error).into()
            })
            .qs_config(serde_qs::Config::new(5, false));

        App::new()
            .wrap(Logger::default())
            .wrap(middleware::cors::default())
            .app_data(json_config.to_owned())
            .app_data(qs_config)
            .app_data(query_config.to_owned())
            .app_data(path_config.to_owned())
            .app_data(repositories.to_owned())
            .configure(swagger::routes::init_routes)
            .configure(health::routes::init_routes)
    })
    .bind(web_addr)?
    .run()
    .await?;

    Ok(())
}
