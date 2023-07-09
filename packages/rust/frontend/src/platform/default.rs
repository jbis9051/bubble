use crate::promise::Promise;

pub type DevicePromise = DefaultPromise;

pub struct DefaultPromise {}

impl Promise for DefaultPromise {
    fn resolve(self, value: &str) {
        println!("resolve: {}", value);
    }

    fn reject(self, value: &str) {
        println!("reject: {}", value);
    }
}

pub mod export {
    use crate::call as call_impl;
    use crate::init as init_impl;
    use crate::platform::DevicePromise;

    pub fn init(data_directory: String) {
        let promise = DevicePromise {};
        init_impl::init(promise, data_directory).unwrap();
    }

    #[no_mangle]
    pub fn call(json: String) {
        let promise = DevicePromise {};
        call_impl::call(promise, &json);
    }
}
