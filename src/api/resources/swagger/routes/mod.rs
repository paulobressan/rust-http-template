use actix_web::web;

pub mod config;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(config::swagger());
    config.service(config::redirect);
}