use crate::js_interface::dynamic_call;
use crate::platform::DevicePromise;
use crate::VIRTUAL_MEMORY;
use serde_json::Value;

pub fn call(promise: DevicePromise, json: &str) {
    let mut deserialized: Value = serde_json::from_str(json).unwrap();
    let instance = deserialized["instance"].as_u64().unwrap();
    let method = deserialized["method"].as_str().unwrap().to_string();
    let args = deserialized["args"].take();
    let instance = VIRTUAL_MEMORY.get(instance as usize).unwrap();
    dynamic_call(instance, &method, args, promise).unwrap();
}
