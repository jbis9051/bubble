use serde_json::{Value};
use crate::{dynamic_call};
use crate::platform::DevicePromise;

pub fn call(promise: DevicePromise, json: &str) {
    let mut deserialized: Value = serde_json::from_str(json).unwrap();
    let method = deserialized["method"].as_str().unwrap().to_string();
    let args = deserialized["args"].take();
    dynamic_call(&method, args, promise).unwrap();
}