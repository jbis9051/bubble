pub mod ios;

use std::ffi::c_void;
use std::future::Future;
use serde::Serialize;
use serde_json::json;

pub use ios::IOSPromise as DevicePromise;

pub type Callbacker = *const c_void;

pub trait Promise {
    fn new(callbacker: Callbacker) -> Self;
    fn resolve(&self, value: &str);
    fn reject(&self, value: &str);
}

pub fn promisify<T: Serialize, E: Serialize>(promise: DevicePromise, f: impl Future<Output = Result<T, E>>) -> impl Future<Output = ()>  {
    async move {
        let result = f.await;
        match result {
            Ok(value) => {
                let value = json!({
                "success": true,
                "value": value
            });
                promise.resolve(&value.to_string());
            }
            Err(error) => {
                let value = json!({
                "success": false,
                "value": error
            });
                promise.reject(&value.to_string());
            }
        };
    }
}