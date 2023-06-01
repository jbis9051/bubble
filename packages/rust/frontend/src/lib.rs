mod export_macro;
pub mod init;
mod models;
mod types;

use crate::init::TokioThread;
use once_cell::sync::{Lazy, OnceCell};
use serde_json::Value;
use sqlx::SqlitePool;
use std::ffi::{c_char, CStr};
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

#[no_mangle]
pub unsafe extern "C" fn init(data_directory: *const c_char) {
    let data_directory = unsafe { CStr::from_ptr(data_directory) }
        .to_str()
        .unwrap()
        .to_string();

    init::init(data_directory).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn call(json: *const c_char) {
    if json.is_null() {
        panic!("rust call function was passed a null pointer");
    }
    let json = unsafe { CStr::from_ptr(json) };
    let json = json.to_str().unwrap();
    let mut deserialized: Value = serde_json::from_str(json).unwrap();
    let method = deserialized["method"].as_str().unwrap().to_string();
    let params = deserialized["params"].take();
    dynamic_call(&method, params).unwrap();
}

pub async fn foo(_abc: String) {
    let abc = reqwest::get("bubble.whatever/user/register")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Hello from foo: {}!", abc);
}

export!(foo(abc: String),);
