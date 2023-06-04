mod export_macro;
pub mod init;
mod models;
mod types;
mod promise;

use crate::init::TokioThread;
use once_cell::sync::{Lazy, OnceCell};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::ffi::{c_char, c_void, CStr};
use std::fmt::format;
use std::thread;
use std::thread::sleep;
use serde::{Serialize, Serializer};
use tokio::sync::RwLock;
use crate::promise::{DevicePromise, Promise};
use bridge_macro::bridge;

#[no_mangle]
pub unsafe extern "C" fn rust_foo(callbacker: *const c_void) {
    let tokio = TokioThread::spawn();
    let promise = DevicePromise::new(callbacker);
    thread::spawn(move || {
        promise.resolve("Hello from a rust Promise!");
    });
}

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

#[no_mangle]
pub unsafe extern "C" fn init(callbacker: *const c_void, data_directory: *const c_char) {
    let data_directory = unsafe { CStr::from_ptr(data_directory) }
        .to_str()
        .unwrap()
        .to_string();

    let promise = DevicePromise::new(callbacker);
    init::init(promise, data_directory).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn call(callbacker: *const c_void, json: *const c_char) {
    if json.is_null() {
        panic!("rust call function was passed a null pointer");
    }
    let json = unsafe { CStr::from_ptr(json) };
    let json = json.to_str().unwrap();
    let mut deserialized: Value = serde_json::from_str(json).unwrap();
    let method = deserialized["method"].as_str().unwrap().to_string();
    let args = deserialized["args"].take();
    let promise = DevicePromise::new(callbacker);
    dynamic_call(&method, args, promise).unwrap();
}

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
#[derive(Serialize)]
#[bridge]
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
    foo(abc: String) -> Result<(), ()>;
    multiply(a: i32, b: i32) -> Result<i32, ()>;
    hello(name: String) -> Result<HelloResponse, ()>;
);