use crate::public::promise::Promise;
use std::sync;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

pub type DevicePromise = DefaultPromise;

static RECEIVER: Mutex<Option<Receiver<String>>> = Mutex::new(None);

pub struct DefaultPromise {
    pub sender: sync::mpsc::Sender<String>,
}

impl DefaultPromise {
    pub fn new() -> Self {
        let (sender, receiver) = sync::mpsc::channel();
        *RECEIVER.lock().unwrap() = Some(receiver);
        DefaultPromise { sender }
    }
}

impl Promise for DefaultPromise {
    fn resolve(self, value: &str) {
        self.sender.send(value.to_string()).unwrap();
    }

    fn reject(self, value: &str) {
        self.sender.send(value.to_string()).unwrap();
    }
}

pub mod export {
    use crate::platform::default::RECEIVER;
    use crate::platform::DevicePromise;
    use crate::public::call as call_impl;
    use crate::public::init as init_impl;

    pub fn init(json: String) {
        let promise = DevicePromise::new();
        init_impl::init(promise, json).unwrap();
    }

    #[no_mangle]
    pub fn call(json: String) {
        let promise = DevicePromise::new();
        call_impl::call(promise, &json);
    }

    pub fn await_fn() -> String {
        let receiver = RECEIVER.lock().unwrap().take().unwrap();
        receiver.recv().unwrap()
    }
}
