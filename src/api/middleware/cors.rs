use actix_cors::Cors;

pub fn default() -> Cors {
    Cors::permissive().max_age(3600)
}
