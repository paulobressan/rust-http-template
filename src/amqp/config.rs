use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref AMQP_CONFIG: Config = Config::from_env();
}

pub fn get_config() -> &'static Config {
    &AMQP_CONFIG
}

#[derive(Debug, Clone)]
pub struct Config {
    pub amqp_addr: String,
}

impl Config {
    fn from_env() -> Self {
        Self {
            amqp_addr: env::var("AMQP_ADDR").expect("AMQP_ADDR must be set"),
        }
    }
}
