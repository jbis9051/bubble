use crate::public::native_api::NativeApi;
use crate::public::promise::Promise;
use jni::objects::{GlobalRef, JObject};
use jni::JavaVM;

pub struct AndroidPromise {
    vm: JavaVM,
    promise: GlobalRef,
}

impl AndroidPromise {
    pub fn new(vm: JavaVM, promise: GlobalRef) -> AndroidPromise {
        AndroidPromise { vm, promise }
    }
}

impl Promise for AndroidPromise {
    fn resolve(self, value: &str) {
        let mut env = self
            .vm
            .attach_current_thread_as_daemon()
            .expect("Couldn't attach thread");
        let value: JObject = env
            .new_string(value)
            .expect("Couldn't create java string!")
            .into();
        env.call_method(
            self.promise,
            "resolve",
            "(Ljava/lang/Object;)V",
            &[value.as_ref().into()],
        )
        .expect("Couldn't call resolve");
    }

    fn reject(self, value: &str) {
        let mut env = self
            .vm
            .attach_current_thread_as_daemon()
            .expect("Couldn't attach thread");
        let value: JObject = env
            .new_string(value)
            .expect("Couldn't create java string!")
            .into();
        env.call_method(
            self.promise,
            "resolve",
            "(Ljava/lang/Object;)V",
            &[value.as_ref().into()],
        )
        .expect("Couldn't call resolve");
    }
}

pub struct AndroidApi {}

impl NativeApi for AndroidApi {
    type Error = ();

    fn init() -> Self {
        Self {}
    }

    fn request_location_permissions(&self) -> Result<bool, Self::Error> {
        println!("request_location_permissions");
        Ok(true)
    }

    fn has_location_permissions(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn subscribe_to_location_updates(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn unsubscribe_from_location_updates(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub type DevicePromise = AndroidPromise;
pub type DeviceApi = AndroidApi;

pub mod export {
    use crate::platform::DevicePromise;
    use crate::public::call::call;
    use crate::public::init::init;
    use jni::objects::{JClass, JObject, JString};
    use jni::JNIEnv;

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_bubble_rust_BubbleModule_nativeInit(
        mut env: JNIEnv,
        _: JClass,
        data_dir: JString,
        promise: JObject,
    ) {
        let promise = DevicePromise::new(
            env.get_java_vm().expect("Couldn't get vm"),
            env.new_global_ref(promise).unwrap(),
        );
        let output: String = env
            .get_string(&data_dir)
            .expect("Couldn't get string")
            .into();
        init(promise, output).expect("Couldn't init");
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_bubble_rust_BubbleModule_nativeCall(
        mut env: JNIEnv,
        _: JClass,
        json: JString,
        promise: JObject,
    ) {
        let promise = DevicePromise::new(
            env.get_java_vm().expect("Couldn't get vm"),
            env.new_global_ref(promise).unwrap(),
        );
        let json: String = env.get_string(&json).expect("Couldn't get string").into();
        call(promise, &json);
    }
}
