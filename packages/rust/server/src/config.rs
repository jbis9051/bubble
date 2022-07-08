use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub listen_addr: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    listen_addr: env::var("LISTEN_ADDR").unwrap(),
});
