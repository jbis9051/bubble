use crate::public::native_api::NativeApi;
use crate::public::promise::{Callbacker, Promise};
use log::warn;
use std::ffi::{c_char, c_void, CString};

pub type DevicePromise = IOSPromise;
pub type DeviceApi = IOSApi;

extern "C" {
    pub fn promise_callbacker_resolve(callbacker: Callbacker, value: *const c_char);
    pub fn promise_callbacker_reject(callbacker: Callbacker, value: *const c_char);

    pub fn create_location_manager() -> *const c_void;
    pub fn request_location_permissions(location_manager: *const c_void);
    pub fn has_location_permissions() -> bool;
    pub fn subscribe_to_location_updates(location_manager: *const c_void);
    pub fn unsubscribe_from_location_updates(location_manager: *const c_void);
    pub fn destroy_location_manager(location_manager: *const c_void);
}

// actual pet peeve of mine having the "i" in "iOS" capitalized but rust complains and
// my google search didn't return a way to disable it so fuck you
pub struct IOSPromise {
    callbacker: *const c_void,
}

unsafe impl Send for IOSPromise {}

impl IOSPromise {
    pub fn new(callbacker: Callbacker) -> Self {
        Self { callbacker }
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

pub struct IOSApi {
    location_manager: *const c_void,
}

unsafe impl Send for IOSApi {}
unsafe impl Sync for IOSApi {}

pub enum IOSApiError {}

impl NativeApi for IOSApi {
    type Error = IOSApiError;

    fn init() -> Self {
        Self {
            location_manager: unsafe { create_location_manager() },
        }
    }

    fn request_location_permissions(&self) -> Result<bool, Self::Error> {
        unsafe { request_location_permissions(self.location_manager) };
        Ok(true)
    }

    fn has_location_permissions(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { has_location_permissions() })
    }
    fn subscribe_to_location_updates(&self) -> Result<(), Self::Error> {
        unsafe { subscribe_to_location_updates(self.location_manager) };
        warn!("did subscribe_to_location_updates");
        Ok(())
    }
    fn unsubscribe_from_location_updates(&self) -> Result<(), Self::Error> {
        unsafe { unsubscribe_from_location_updates(self.location_manager) };
        Ok(())
    }
}

impl Drop for IOSApi {
    fn drop(&mut self) {
        unsafe { destroy_location_manager(self.location_manager) };
    }
}

pub mod export {
    use crate::platform::ios::DevicePromise;
    use crate::public::background_location_update::BackgroundLocationUpdateOptions;
    use crate::public::{
        background_location_update as background_location_update_impl, call as call_impl,
        init as init_impl,
    };
    use log::warn;
    use std::ffi::{c_char, c_void, CStr};
    use std::fs::File;
    use std::io::Write;

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

    #[no_mangle]
    pub unsafe extern "C" fn background_location_update(json: *const c_char) {
        warn!("background_location_update");
        if json.is_null() {
            panic!("rust background_location_update function was passed a null pointer");
        }
        let json = unsafe { CStr::from_ptr(json) };
        let json = json.to_str().unwrap().to_string();
        warn!("will parse: {:?}", json);
        let options: BackgroundLocationUpdateOptions = serde_json::from_str(&json)
            .map_err(|e| {
                warn!("error parsing json: {:?}", e);
                e
            })
            .unwrap();
        background_location_update_impl::background_location_update(options);
    }
}
