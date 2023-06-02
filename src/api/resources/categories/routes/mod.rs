use actix_web::web;

pub mod create;
pub mod delete_by_id;
pub mod find;
pub mod find_by_id;
pub mod update_by_id;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create::handler);
    config.service(update_by_id::handler);
    config.service(find_by_id::handler);
    config.service(find::handler);
    config.service(delete_by_id::handler);
}
