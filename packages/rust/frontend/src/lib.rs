mod export_macro;

use once_cell::sync::OnceCell;
use serde_json::Value;
use std::ffi::{c_char, CStr};
use std::sync;
use std::thread;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct TokioThread {
    handle: Handle,
    shutdown: Sender<()>,
}

static TOKIO_THREAD: OnceCell<TokioThread> = OnceCell::new();

#[no_mangle]
pub extern "C" fn init() {
    let (handle_send, handle_recv) = sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    thread::spawn(move || {
        let runtime = Runtime::new().unwrap();
        handle_send.send(runtime.handle().clone()).unwrap();
        runtime.block_on(async {
            shutdown_rx.await.unwrap();
        });
    });
    let handle = handle_recv.recv().unwrap();
    TOKIO_THREAD
        .set(TokioThread {
            handle,
            shutdown: shutdown_tx,
        })
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn call(json: *const c_char) {
    if json.is_null() {
        panic!("rust call function was passed a null pointer");
    }
    let json = unsafe { CStr::from_ptr(json) };
    let json = json.to_str().unwrap();
    let mut deserialized: Value = serde_json::from_str(json).unwrap();
    let method = deserialized["method"].as_str().unwrap().to_string();
    let params = deserialized["params"].take();
    dynamic_call(&method, params, &TOKIO_THREAD).unwrap();
}

pub async fn foo(_abc: String) {
    let abc = reqwest::get("bubble.whatever/user/register")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Hello from foo: {}!", abc);
}

export!(foo(abc: String),);
