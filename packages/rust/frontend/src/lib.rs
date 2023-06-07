mod call;
mod export_macro;
mod init;
mod models;
mod platform;
mod promise;
mod types;

// export all platform specific functions
pub use platform::export::*;

use crate::init::TokioThread;
use bridge_macro::bridge;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("sqlx migrate error: {0}")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error("global oneshot initialized, you probably called init twice")]
    GlobalAlreadyInitialized,
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = json!({
            "message": self.to_string()
        });
        value.serialize(serializer)
    }
}

#[derive(Debug)]
pub struct GlobalStaticData {
    pub data_directory: String,
    pub tokio: TokioThread,
}

#[derive(Debug)]
pub struct GlobalAccountData {
    pub bearer: RwLock<String>,
    pub database: SqlitePool,
}

pub static GLOBAL_STATIC_DATA: OnceCell<GlobalStaticData> = OnceCell::new();
pub static GLOBAL_DATABASE: OnceCell<SqlitePool> = OnceCell::new();
pub static GLOBAL_ACCOUNT_DATA: Lazy<RwLock<Option<GlobalAccountData>>> =
    Lazy::new(|| RwLock::new(None));

pub async fn foo(_abc: String) -> Result<(), ()> {
    let abc = reqwest::get("bubble.whatever/user/register")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Hello from foo: {}!", abc);

    Ok(())
}

#[bridge]
pub async fn multiply(a: i32, b: i32) -> Result<i32, ()> {
    Ok(a * b)
}

#[bridge]
#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

#[bridge]
pub async fn hello(name: String) -> Result<HelloResponse, ()> {
    Ok(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}

export!(
    multiply(a: i32, b: i32) -> Result<i32, ()>;
    hello(name: String) -> Result<HelloResponse, ()>;
);
