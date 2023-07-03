use frontend::application_message::Location;
use frontend::init;
use frontend::js_interface::group::Group;
use serde::Deserialize;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
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
            println!("{:?}", serde_json::to_string_pretty(&res.value).unwrap());
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

    println!("1");

    let alice_uuid = call!(alice_instance, login(username_or_email: "aliceusername", password: "alicepassword") -> Result<Uuid, ()>).unwrap();
    let bob_uuid = call!(bob_instance, login(username_or_email: "bobusername", password: "bobpassword") -> Result<Uuid, ()>).unwrap();

    println!("2");

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 0);

    println!("3");

    let group_uuid = call!(alice_instance, create_group() -> Result<Uuid, ()>).unwrap();

    println!("4");

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].uuid, group_uuid);
    assert_eq!(groups[0].name, None);
    assert_eq!(groups[0].image, None);
    assert_eq!(groups[0].members.len(), 1);
    assert!(groups[0].members.get(&alice_uuid).is_some());

    println!("5");

    call!(
        alice_instance,
        add_member(group_uuid: group_uuid, user_uuid: bob_uuid)
    )
    .unwrap();

    println!("6");

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].members.len(), 2);
    assert!(groups[0].members.get(&bob_uuid).is_some());
    assert_eq!(groups[0].members.get(&bob_uuid).unwrap().len(), 1);
    let bob_client = groups[0].members.get(&bob_uuid).unwrap()[0];

    println!("7");

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 0);

    call!(bob_instance, receive_messages()).unwrap();

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].members.len(), 2);
    assert!(groups[0].members.get(&bob_uuid).is_some());
    assert!(groups[0].members.get(&alice_uuid).is_some());
    assert_eq!(groups[0].members.get(&alice_uuid).unwrap().len(), 1);

    let future = i64::MAX;
    let locations = call!(bob_instance, get_location(group_uuid: group_uuid, client: bob_client, before_timestamp: future, amount: 100) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(bob_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(locations.len(), num_locations as usize);
    assert_eq!(num_locations, 0);

    let locations = call!(alice_instance, get_location(group_uuid: group_uuid, client: bob_client, before_timestamp: future, amount: 1) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(alice_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(locations.len(), num_locations as usize);
    assert_eq!(num_locations, 0);

    let alice_location = (37.2431, -115.7930);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(alice_instance, send_location(group_uuid: group_uuid, longitude: alice_location.0, latitude: alice_location.1, timestamp: now)).unwrap();

    call!(bob_instance, receive_messages()).unwrap();

    let locations = call!(bob_instance, get_location(group_uuid: group_uuid, client: alice_uuid, before_timestamp: future, amount: 100) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(bob_instance, get_num_location(group_uuid: group_uuid, client: alice_uuid, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(locations.len(), num_locations as usize);
    assert_eq!(num_locations, 1);
    assert_eq!(locations[0].longitude, alice_location.0);
    assert_eq!(locations[0].latitude, alice_location.1);
    assert_eq!(locations[0].timestamp, now);

    let bob_location = (32.0853, 34.7818);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(bob_instance, send_location(group_uuid: group_uuid, longitude: bob_location.0, latitude: bob_location.1, timestamp: now)).unwrap();

    call!(alice_instance, receive_messages()).unwrap();

    let locations = call!(alice_instance, get_location(group_uuid: group_uuid, client: bob_uuid, before_timestamp: future, amount: 100) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(alice_instance, get_num_location(group_uuid: group_uuid, client: bob_uuid, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(locations.len(), num_locations as usize);
    assert_eq!(num_locations, 1);
    assert_eq!(locations[0].longitude, bob_location.0);
    assert_eq!(locations[0].latitude, bob_location.1);
    assert_eq!(locations[0].timestamp, now);
}
