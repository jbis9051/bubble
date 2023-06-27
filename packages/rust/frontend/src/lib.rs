mod api;
mod application_message;
mod helper;
mod js_interface;
mod mls_provider;
mod models;
mod platform;
mod public;
mod types;
mod virtual_memory;

use once_cell::sync::Lazy;
use std::sync::Arc;
// export all platform specific functions
pub use platform::export::*;

use crate::js_interface::FrontendInstance;
use crate::virtual_memory::VirtualMemory;
use serde::{Serialize, Serializer};
use serde_json::json;
use sqlx::migrate::MigrateError;

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

pub static VIRTUAL_MEMORY: Lazy<VirtualMemory<Arc<FrontendInstance>>> =
    Lazy::new(VirtualMemory::new);
