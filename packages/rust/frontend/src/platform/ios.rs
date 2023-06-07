use std::ffi::{c_char, c_void, CString};
use crate::promise::{Callbacker, Promise};

pub type DevicePromise = IOSPromise;

extern "C" {
    pub fn promise_callbacker_resolve(callbacker: Callbacker, value: *const c_char);
    pub fn promise_callbacker_reject(callbacker: Callbacker, value: *const c_char);
}

// actual pet peeve of mine having the "i" in "iOS" capitalized but rust complains and
// my google search didn't return a way to disable it so fuck you
pub struct IOSPromise {
    callbacker: *const c_void,
}

unsafe impl Send for IOSPromise{}

impl IOSPromise {
    pub fn new(callbacker: Callbacker) -> Self {
        Self {
            callbacker
        }
    }
}

impl Promise for IOSPromise {
    fn resolve(self, value: &str) {
        let value = CString::new(value).unwrap();
        let value = value.into_raw();
        unsafe { promise_callbacker_resolve(self.callbacker, value) };
    }

    fn reject(self, value: &str) {
        let value = CString::new(value).unwrap();
        let value = value.into_raw();
        unsafe { promise_callbacker_reject(self.callbacker, value) };
    }
}

pub mod export {
    use std::ffi::{c_char, c_void, CStr};
    use crate::{call as call_impl, init as init_impl};
    use crate::platform::ios::DevicePromise;

    #[no_mangle]
    pub unsafe extern "C" fn init(callbacker: *const c_void, data_directory: *const c_char) {
        let data_directory = unsafe { CStr::from_ptr(data_directory) }
            .to_str()
            .unwrap()
            .to_string();

        let promise = DevicePromise::new(callbacker);
        init_impl::init(promise, data_directory).unwrap();
    }

    #[no_mangle]
    pub unsafe extern "C" fn call(callbacker: *const c_void, json: *const c_char) {
        if json.is_null() {
            panic!("rust call function was passed a null pointer");
        }
        let json = unsafe { CStr::from_ptr(json) };
        let json = json.to_str().unwrap();
        let promise = DevicePromise::new(callbacker);
        call_impl::call(promise, json);
    }
}