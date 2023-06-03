use std::ffi::{c_char, c_void, CString};
use crate::promise::{Callbacker, Promise};


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

impl Promise for IOSPromise {
    fn new(callbacker: Callbacker) -> Self {
        Self {
            callbacker
        }
    }

    fn resolve(&self, value: &str) {
        let value = CString::new(value).unwrap();
        let value = value.into_raw();
        unsafe { promise_callbacker_resolve(self.callbacker, value) };
    }

    fn reject(&self, value: &str) {
        let value = CString::new(value).unwrap();
        let value = value.into_raw();
        unsafe { promise_callbacker_reject(self.callbacker, value) };
    }
}