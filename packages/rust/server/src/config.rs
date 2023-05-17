use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub listen_addr: String,
    pub db_url: String,
    pub api_key_check: String,
    pub sender_email: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    listen_addr: env::var("LISTEN_ADDR").unwrap(),
    db_url: env::var("DB_URL").unwrap(),
    api_key_check: env::var("SENDGRID_API_KEY").unwrap(), // pull api key from env. variables
    sender_email: env::var("sender_email").unwrap(),
});
