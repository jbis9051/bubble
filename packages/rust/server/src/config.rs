use once_cell::sync::Lazy;
use std::env;

struct Config {
    listen_addr: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    listen_addr: env::var("LISTEN_ADDR").unwrap(),
});
