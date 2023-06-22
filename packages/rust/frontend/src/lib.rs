mod api;
mod helper;
mod js_interface;
mod mls_provider;
mod models;
mod platform;
mod public;
mod types;

// export all platform specific functions
pub use platform::export::*;

use crate::public::init::TokioThread;
use bridge_macro::bridge;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Serialize, Serializer};
use serde_json::json;
use sqlx::migrate::MigrateError;
use sqlx::SqlitePool;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("sqlx migrate error: {0}")]
    SqlxMigrate(#[from] MigrateError),
    #[error("global oneshot initialized, you probably called init twice")]
    GlobalAlreadyInitialized,
    #[error("don't know what to return for this error yet")]
    TestingError,
    #[error("unable to parse uuid for field '{0}': {1}")]
    UuidParseError(&'static str, uuid::Error),
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
    pub database: SqlitePool,
    pub bearer: RwLock<String>,            // cached value
    pub domain: String,                    // cached value
    pub user_uuid: Uuid,                   // cached value
    pub client_uuid: RwLock<Option<Uuid>>, // cached value
}

pub static GLOBAL_STATIC_DATA: OnceCell<GlobalStaticData> = OnceCell::new();
pub static GLOBAL_DATABASE: OnceCell<SqlitePool> = OnceCell::new();
pub static GLOBAL_ACCOUNT_DATA: Lazy<RwLock<Option<GlobalAccountData>>> =
    Lazy::new(|| RwLock::new(None));

#[bridge]
pub async fn multiply(a: i32, b: i32) -> Result<i32, ()> {
    Ok(a * b)
}

export!(
    multiply(a: i32, b: i32) -> Result<i32, ()>;
);
