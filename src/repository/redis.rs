use std::env;

use redis::Client;

pub fn init() -> Client {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    redis::Client::open(redis_url).expect("Error to init redis client")
}
