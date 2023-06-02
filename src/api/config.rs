use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref CONFIG: Config = Config::from_env();
}

pub fn get_config() -> &'static Config {
    &CONFIG
}

#[derive(Debug, Clone)]
pub struct Config {
    pub web_addr: String,
    pub page_size_default: u32,
    pub page_size_max: u32,
}

impl Config {
    fn from_env() -> Self {
        Self {
            web_addr: env::var("ADDR").expect("ADDR must be set"),
            page_size_default: env::var("PAGE_SIZE_DEFAULT")
                .expect("PAGE_SIZE_DEFAULT must be set")
                .parse()
                .expect("PAGE_SIZE_DEFAULT must be u32"),
            page_size_max: env::var("PAGE_SIZE_MAX")
                .expect("PAGE_SIZE_MAX must be set")
                .parse::<u32>()
                .expect("PAGE_SIZE_MAX must be u32"),
        }
    }
}
