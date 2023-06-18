I figure I should document how this thing works at least a little bit due to its complexity.

So let's first describe the problem we are solving. React Native is used for the UI in Bubble. However, we don't want to
write the frontend logic in JavaScript/TypeScript because...well...it's JavaScript. If you want a real reason,
the MLS library we are using is in Rust. So we might as well write it all in Rust.

React Native does support writing native modules, however the native modules must be written in the platform's native language.
For iOS, this is Objective-C. For Android, this is Java. This presents our first challenge: How do we get Rust and
Objective-C/Java to talk ~~nicely~~?

The Objective-C side is relatively straight forward. Objective-C is compatible with the C ABI. So all we have to do is
build a static library from our Rust code, and then link the Objective-C code against our static library. All Rust functions
exposed to Objective-C must be annotated with `#[no_mangle]` and `extern "C"` to tell Rust to make them compatible with
the C ABI. The Objective-C code is in `js/bubble_rust/ios`.

The Java side is a bit more annoying. Java is compatible with the C ABI through JNI but it's a bit painful to use. Luckily, Rust
has a JNI crate, so we'll use that. The Java code is in `js/bubble_rust/android`.

First problem is solved. Moving on...

Every native function that is callable from React Native must be defined in the native language. This is painful because
it means that we have to write every signature four times: once in Rust, once in Objective-C, once in Java, and once
in TypeScript. Then we have to convert all the parameters back and forth. Yikes!

We eventually will probably want to use some type of code generation tool to solve this problem. However, for now we use a bit of a hack to avoid it. We define a single function, `call`, in Rust, Objective-C, Java, and TypeScript. This function
accepts a single string parameter which is a JSON string. The JSON string contains two keys. The first key is `method` which
is the name of the function to call. The second key is `args` which is an array of parameters to pass to the function.

For example, if you wanted to call the `add` function, you would call `call` with the following JSON string:

```json
{
  "method": "add",
  "args": [1, 2]
}
```

Rust parses the JSON string and dynamically calls the appropriate function. The return value is similarly converted to a JSON
string and returned to the caller.

On the Rust side, we use serde to both serialize and deserialize the JSON string. This is all transparent to
Objective-C/Java which simply accepts a string and returns a string. The TypeScript layer handles serializing the calls and
deserializing the return values.

We've eliminated needing to write the signatures in Objective-C/Java, but we still need to write them in Rust and TypeScript.
We can do better.

Since TypeScript types are relatively simple and the only type conversion code is JSON serialization/deserialization, we can
use code generation to generate the TypeScript types from the Rust types. The system has two components. The first is the bridge_macro (in rust/bridge_macro).
This is a procedural macro that does absolutely nothing. But it allows us to mark functions with the `#[bridge]` attribute for use later.
We can also add the `#[bridge]` attribute to structs so functions can return custom types.

The second component is the bridge generator (in rust/bridge_gen). This is a simple CLI app that actually generates the
TypeScript types. It iterates through all .rs files in a given directly, and uses the `syn` crate to parse the Rust code
in each of those files searching for the `#[bridge]` attribute. Once it fines one it runs some logic to generate the
TypeScript types. Most of the conversions are pretty simple, Rust u8, i16, f32, etc. all map to TypeScript number.
Collections such as `Vec<T>` are converted to `T[]` aka JavaScript arrays. Structs are converted to TypeScript interfaces.
We then recursively iterate over the fields and convert them to TypeScript types.

The only tricky part is Rust `Result`s because they are a union type and used too frequently to simply avoid.

To solve this, we define a custom TypeScript type called `Result`:

```typescript
// packages/js/bubble_rust/src/index.ts
export type Result<T, E>  = {
    success: true;
    value: T;
} | {
    success: false;
    value: E;
}
```

Basically, we use `success` as the discriminator. If `success` is true, then the `value` field is the success value, T,
otherwise it's the error value, E.

On the Rust side we simply convert a given `Result` to JSON that matches the above TypeScript type. We'll talk more
about this process later.

Once it's all done, the bridge generator then writes the TypeScript types to `packages/js/bubble_rust/src/gen.ts`. This
file is then reexported from `packages/js/bubble_rust/src/index.ts` so we can add any extra types we want and import them
from our React Native code.

So for something like

```rust
#[bridge]
pub async fn multiply(a: i32, b: i32) -> Result<i32, ()> {
    Ok(a * b)
}

#[bridge]
#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

#[bridge]
pub async fn hello(name: String) -> Result<HelloResponse, ()> {
    Ok(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}
```

The TypeScript interface generated is:

```typescript

export interface HelloResponse {
    message: string,
}

export function multiply(a: number , b: number ): Promise<Result<number, void>> {
    return RustInterop.call(JSON.stringify({
        method: 'multiply',
        args: {a, b},
    })).then((res: string) => JSON.parse(res));
}

export function hello(name: string ): Promise<Result<HelloResponse, void>> {
    return RustInterop.call(JSON.stringify({
        method: 'hello',
        args: {name},
    })).then((res: string) => JSON.parse(res));
}
```

Great, now we only have to write the signatures once in Rust. Moving on.

If we read the React Native documentation, we see this interesting tid bit:

> Unless the native module provides its own method queue, it shouldn't make any assumptions about what thread it's being called on. Currently, if a native module doesn't provide a method queue, React Native will create a separate GCD queue for it and invoke its methods there. Please note that this is an implementation detail and might change. If you want to explicitly provide a method queue for a native module, override the (dispatch_queue_t) methodQueue method in the native module.

Well shit. Given that we are going to run expensive crypto operations, we can't have our Rust functions on the main thread.
Spawning a bunch of OS threads is kinda expensive. Luckily, we can use async. Rust has decent async support through Tokio.
JavaScript uses the Promise interface. From MDN

> The Promise object represents the eventual completion (or failure) of an asynchronous operation and its resulting value.

Basically you can `await` a Promise on the JS and it'll asynchronously wait for Promise to either be resolved (finished successfully)
or rejected (failed).

Cool. So how do we connect these two interfaces? Lucky for us React Native provides a native interface for interacting with
JS Promises. The problem is these native interfaces are once again in the native languages Objective-C/Java. Grr.

In Objective-C, React Native provides two types `RCTPromiseResolveBlock` and `RCTPromiseRejectBlock`. These can be called
within Objective-C to resolve or reject a Promise. They can even be stored long term for async operations. Nifty.

While Objective-C can easily call C functions, C can't easily call methods on Objective-C objects. Furthermore, I was unable
to find a single shred of public documentation for these types. The best I found was some internal React Native code that
suggested that the `RCTPromiseResolveBlock` type were actually just typedefs for something close `void (^)(id)`. I'm pretty
sure ^ denotes a block type in Objective-C. I couldn't seem to figure out how to call a block from over the FFI boundary though. I'm not sure
if it's possible.

Luckily, Objective-C has a means to expose C ABI functions. So I figured we could probably have a C function call
`RCTPromiseResolveBlock` and `RCTPromiseRejectBlock` for us. The basic system is we define a new Objective-C class `Callbacker`
which stores a `RCTPromiseResolveBlock` and `RCTPromiseRejectBlock` as instance variables. We then define two C function in Objective-C:

```c
// packages/js/bubble_rust/ios/RCTBubble.m
void promise_callbacker_resolve(const void *self, const char *result);
void promise_callbacker_reject(const void *self, const char *error);
```

void* in C is basically a pointer to anything so we can use it to pass a pointer to a `Callbacker` instance transparently to the call.
We then cast the void* to a `Callbacker` instance and call the appropriate method. The `result` and `error` parameters are
just regular C strings (in this case JSON strings).

So it works like this: When we call a Rust function from Objective-C, we pass a `Callbacker` instance casted to a void* to the Rust function. The Rust function
eventually calls `promise_callbacker_resolve` or `promise_callbacker_reject` with the `void*` Callbacker instance. The C function
then calls the appropriate method on the `Callbacker` instance member variables. React Native takes it from there and either resolves or rejects the Promise
on the JS side.

The Objective-C code looks like this:

```objective-c
// RCT_REMAP_METHOD is a React Native macro that allows us to call the function from JS
RCT_REMAP_METHOD(call, callWithJson
                 : (NSString *)json withResolver
                 : (RCTPromiseResolveBlock)resolve withRejecter
                 : (RCTPromiseRejectBlock)reject) {

  Callbacker *callbacker = [[Callbacker alloc] initWithResolve:resolve
                                                        reject:reject]; // initialize the Callbacker instance with resolve (RCTPromiseResolveBlock) and reject (RCTPromiseRejectBlock)
  call((void *)(callbacker), [json UTF8String]); // call the Rust function with the Callbacker instance casted to a void* and the json string
}
```

The first time I did this, it crashed. Damn. After some debugging, without any ability to log anything or view crash logs
(because React Native crap), I figured out that it was a classic use-after-free bug. The `Callbacker` instance was being
freed after the initial call to the Rust function. This kinda made sense. Since the `Callbacker` was created in
the `RCT_REMAP_METHOD(call)` function it was being freed when the function returned. Objective-C has no idea that `call`
would hold onto the pointer to the `Callbacker` instance after it returned. Which it does, because `call` is async.

Luckily, Objective-C has a solution for this. We simply use the `CFBridgingRetain` method to retain the `Callbacker` instance
and then use `CFBridgingRelease` to release it when we are done with it. This tells Objective-C not to GC the `Callbacker` by manually
incrementing the reference count. So we do:

```objective-c
  call((void *)CFBridgingRetain(callbacker), [json UTF8String]); // increment the reference count
```

then in `promise_callbacker_resolve` and `promise_callbacker_reject` we do:

```c
  Callbacker *callbacker = (Callbacker *)CFBridgingRelease(self); // decrement the reference count
```

Finally, on the Rust side we provide a simple Promise interface

```rust
// packages/rust/frontend/src/promise.rs
pub type Callbacker = *const c_void;

pub trait Promise {
    fn new(callbacker: Callbacker) -> Self;
    fn resolve(self, value: &str);
    fn reject(self, value: &str);
}

// the implementation for iOS (packages/rust/frontend/src/platform/ios.rs:
pub struct IOSPromise {
    callbacker: *const c_void,
}
impl Promise for IOSPromise {
    fn resolve(self, value: &str) {
        let value = CString::new(value).unwrap();
        let value = value.into_raw();
        unsafe { promise_callbacker_resolve(self.callbacker, value) };
    }
    ...
 }
```

Last but not least, we must convert async Rust functions to use this Promise interface. This is pretty simple we just use:

```rust
pub async fn promisify<T: Serialize, E: Serialize>(promise: DevicePromise, f: impl Future<Output=Result<T, E>>) {
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
```

This is pretty self-explanatory. We take in a `promise` and some `Future` which resolves to a `Result` (`Futures` are the
Rust equivalent of Promises). We then await the `Future` and match on the result. If the result is Ok, we resolve the `promise`
otherwise we reject it. This is also where the actual conversion from a Rust Result to TypeScript JSON "Result" happens.

For Java we use a very similar method. Instead of `Callback` JNI allows us to call the `resolve` method directly on the `Promise`. We also have to deal with Java's garbage collection system. We can use JNI `GlobalRef`s to tell the JVM GC not to free our `Promise` before we use it. JNI also requires us to connect to the JVM before accessing a `GlobalRef` we do this via the `jni` crates `attach_current_thread_as_daemon` method. 
Finally, we use `jni`'s `call_method` to actually call the resolve method on the `Promise`.
The `jni` implementation actually manages releasing the `GlobalRef` back to the JVM when we drop it for us so we don't need to do anything manual.

```rust
// packages/rust/frontend/src/platform/android.rs
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
    ...
}
```

Cool. Now we have a way for Rust async functions to resolve or reject Promises on the JS side.

Next up, we need to figure out how to have the Rust `call` function actually run whatever method is requested to run
through Tokio. In Tokio, this referred to `spawn`ing a task on the Tokio runtime. Before we can spawn a task, we need
to start a Tokio runtime. This is simple enough. We could have done this lazily, but for some application specific
reasons, I decided to expose a dedicated `init` function that handles this. `init` is manually specified all four times,
like `call` is. The init function spawns a Tokio thread, does some other stuff, and then stores a handle to the
Tokio runtime in a global variable. The `call` function then uses this global variable to spawn tasks.

Now, in call it would be really annoying to manually deserialize the input JSON, manually look for the proper method to call,
call the method, and then convert the output back to JSON.

So instead we use a Rust macro to do this all for us. We call this macro `export!` as it exposes this function to the JS side via the `call` function.

This macro is located in `packages/rust/frontend/src/export_macro.rs`. Using some fancy macro magic, it converts:

```rust
export!(
    multiply(a: i32, b: i32) -> Result<i32, ()>;
    hello(name: String) -> Result<HelloResponse, ()>;
);
```

to

```rust
pub fn dynamic_call(
    name_: &str,
    mut args_: Value, // this is just a JSON object
    promise: crate::promise::DevicePromise,
) -> Result<(), ()> {
    match name_ {
        "multiply" => {
            let a: i32 = serde_json::from_value(args_["a"].take()).unwrap();
            let b: i32 = serde_json::from_value(args_["b"].take()).unwrap();
            crate::GLOBAL_STATIC_DATA.get().unwrap().tokio.handle.spawn(
                crate::promise::promisify::<i32, ()>(promise, crate::multiply(a, b)),
            );
            Ok(())
        }
        "hello" => {
            let name: String = serde_json::from_value(args_["name"].take()).unwrap();
            crate::GLOBAL_STATIC_DATA.get().unwrap().tokio.handle.spawn(
                crate::promise::promisify::<HelloResponse, ()>(promise, crate::hello(name)),
            );
            Ok(())
        }
        _ => Err(()),
    }
```

Looks a bit verbose, but it matches the `name_` to one of the function names in the `export!` macro.
It then deserializes the respective arguments for that method and spawns a task on the Tokio runtime with the
promisified version of the async function.

The `call` function then calls the `dynamic_call` function with the name of the method to call, the arguments, and the promise. Awesome.

And bingo, we have a nice abstraction for asynchronously interfacing between Rust and React Native.

