use frontend::init;
use frontend::js_interface::group::Group;
use serde::Deserialize;
use serde_json::Value;
use std::{env, fs};
use thiserror::__private::PathAsDisplay;
use uuid::Uuid;

#[derive(Deserialize)]
struct PromiseResult {
    success: bool,
    value: Value,
}

macro_rules! call {
    (
        $instance: ident,
        $name: ident(
              $($arg_name: ident: $arg: expr),*
        )
    ) => {
        call!($instance, $name($($arg_name: $arg),*) -> Result<(), ()>)
    };
    (
        $instance: ident,
        $name: ident(
              $($arg_name: ident: $arg: expr),*
        ) -> Result<$return_type: ty, $error_type: ty>
    ) => {
        {
            let json = serde_json::json!({
                "instance": $instance,
                "method": stringify!($name),
                "args": {
                    $(
                        stringify!($arg_name): $arg
                    ),*
                }
            });
            frontend::call(json.to_string());
            await_fn!($return_type, $error_type)
        }
    };
}

macro_rules! await_fn {
    ($return_type: ty, $error_type: ty) => {{
        let res: PromiseResult = serde_json::from_str(&frontend::await_fn()).unwrap();
        if res.success {
            Ok(serde_json::from_value::<$return_type>(res.value).unwrap())
        } else {
            Err(serde_json::from_value::<$error_type>(res.value).unwrap())
        }
    }};
}

pub fn create_instance(name: &str) -> Result<i32, String> {
    let dir = env::temp_dir();
    let alice_dir = format!("{}bubble_{}_{}", &dir.as_display(), name, &Uuid::new_v4());
    fs::create_dir_all(&alice_dir).unwrap();
    init(alice_dir);
    await_fn!(i32, String)
}

#[test]
pub fn e2e_test() {
    let alice_instance = create_instance("alice").unwrap();
    let bob_instance = create_instance("bob").unwrap();

    call!(alice_instance, register(username: "aliceusername", password: "alicepassword", name: "alice", email: "alice@email.com")).unwrap();
    call!(bob_instance, register(username: "bobusername", password: "bobpassword", name: "bob", email: "bob@email.com")).unwrap();

    call!(alice_instance, login(username: "aliceusername", password: "alicepassword")).unwrap();
    call!(bob_instance, login(username: "bobusername", password: "bobpassword")).unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 0);

    let group_uuid = call!(alice_instance, create_group() -> Result<Uuid, ()>).unwrap();
    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].uuid, group_uuid);
}
