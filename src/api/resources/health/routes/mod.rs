use actix_web::web;

pub mod check;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(check::handler);
}
