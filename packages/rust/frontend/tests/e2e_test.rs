use frontend::application_message::Location;
use frontend::init;
use frontend::js_interface::group::Group;
use frontend::public::init::InitOptions;
use serde::Deserialize;
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
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
    let dir = format!("{}bubble_{}_{}", &dir.as_display(), name, &Uuid::new_v4());
    fs::create_dir_all(&dir).unwrap();
    init(
        serde_json::to_string(&InitOptions {
            data_directory: dir,
            force_new: true,
        })
        .unwrap(),
    );
    await_fn!(i32, String)
}

#[test]
pub fn test_basic() {
    reqwest::blocking::get("http://localhost:3000/reset").unwrap();

    let alice_instance = create_instance("alice").unwrap();
    let bob_instance = create_instance("bob").unwrap();

    call!(alice_instance, register(username: "aliceusername", password: "alicepassword", name: "alice", email: "alice@email.com")).unwrap();
    call!(bob_instance, register(username: "bobusername", password: "bobpassword", name: "bob", email: "bob@email.com")).unwrap();

    let alice_uuid = call!(alice_instance, login(username_or_email: "aliceusername", password: "alicepassword") -> Result<Uuid, ()>).unwrap();
    let bob_uuid = call!(bob_instance, login(username_or_email: "bobusername", password: "bobpassword") -> Result<Uuid, ()>).unwrap();

    call!(alice_instance, replace_key_packages()).unwrap();
    call!(bob_instance, replace_key_packages()).unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 0);

    let group_uuid = call!(alice_instance, create_group() -> Result<Uuid, ()>).unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].uuid, group_uuid);
    assert_eq!(groups[0].name, None);
    assert_eq!(groups[0].image, None);
    assert_eq!(groups[0].members.len(), 1);
    assert!(groups[0].members.get(&alice_uuid).is_some());

    call!(
        alice_instance,
        add_member(group_uuid: group_uuid, user_uuid: bob_uuid)
    )
    .unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].members.len(), 2);
    assert!(groups[0].members.get(&bob_uuid).is_some());
    assert_eq!(groups[0].members.get(&bob_uuid).unwrap().clients.len(), 1);
    let bob_client = groups[0].members.get(&bob_uuid).unwrap().clients[0];

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 0);

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].members.len(), 2);
    assert!(groups[0].members.get(&bob_uuid).is_some());
    assert!(groups[0].members.get(&alice_uuid).is_some());
    assert_eq!(groups[0].members.get(&alice_uuid).unwrap().clients.len(), 1);
    let alice_client = groups[0].members.get(&alice_uuid).unwrap().clients[0];

    let future = NaiveDateTime::MAX.timestamp_millis();

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

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let locations = call!(bob_instance, get_location(group_uuid: group_uuid, client: alice_client, before_timestamp: future, amount: 100) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(bob_instance, get_num_location(group_uuid: group_uuid, client: alice_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
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

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let locations = call!(alice_instance, get_location(group_uuid: group_uuid, client: bob_client, before_timestamp: future, amount: 100) -> Result<Vec<Location>, ()>).unwrap();
    let num_locations = call!(alice_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(locations.len(), num_locations as usize);
    assert_eq!(num_locations, 1);
    assert_eq!(locations[0].longitude, bob_location.0);
    assert_eq!(locations[0].latitude, bob_location.1);
    assert_eq!(locations[0].timestamp, now);
}

#[test]
pub fn test_full() {
    reqwest::blocking::get("http://localhost:3000/reset").unwrap();

    let alice_instance = create_instance("alice").unwrap();
    let bob_instance = create_instance("bob").unwrap();
    let charlie_instance = create_instance("charlie").unwrap();

    call!(alice_instance, register(username: "aliceusername", password: "alicepassword", name: "alice", email: "alice@email.com")).unwrap();
    call!(bob_instance, register(username: "bobusername", password: "bobpassword", name: "bob", email: "bob@email.com")).unwrap();
    call!(charlie_instance, register(username: "charlieusername", password: "charliepassword", name: "charlie", email: "charlie@email.com")).unwrap();

    let alice_uuid = call!(alice_instance, login(username_or_email: "aliceusername", password: "alicepassword") -> Result<Uuid, ()>).unwrap();
    let bob_uuid = call!(bob_instance, login(username_or_email: "bobusername", password: "bobpassword") -> Result<Uuid, ()>).unwrap();
    let charlie_uuid = call!(charlie_instance, login(username_or_email: "charlieusername", password: "charliepassword") -> Result<Uuid, ()>).unwrap();

    call!(alice_instance, replace_key_packages()).unwrap();
    call!(bob_instance, replace_key_packages()).unwrap();
    call!(charlie_instance, replace_key_packages()).unwrap();

    let group_uuid = call!(alice_instance, create_group() -> Result<Uuid, ()>).unwrap();

    call!(
        alice_instance,
        add_member(group_uuid: group_uuid, user_uuid: bob_uuid)
    )
    .unwrap();

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    call!(
        alice_instance,
        add_member(group_uuid: group_uuid, user_uuid: charlie_uuid)
    )
    .unwrap();

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let alice_client = groups[0].members.get(&alice_uuid).unwrap().clients[0];
    let bob_client = groups[0].members.get(&bob_uuid).unwrap().clients[0];
    let _charlie_client = groups[0].members.get(&charlie_uuid).unwrap().clients[0];

    let future = NaiveDateTime::MAX.timestamp_millis();

    let alice_location = (37.2431, -115.7930);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(alice_instance, send_location(group_uuid: group_uuid, longitude: alice_location.0, latitude: alice_location.1, timestamp: now)).unwrap();

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(bob_instance, get_num_location(group_uuid: group_uuid, client: alice_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 1);

    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(charlie_instance, get_num_location(group_uuid: group_uuid, client: alice_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 1);

    let bob_location = (32.0853, 34.7818);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(bob_instance, send_location(group_uuid: group_uuid, longitude: bob_location.0, latitude: bob_location.1, timestamp: now)).unwrap();

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(alice_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 1);

    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(charlie_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 1);

    call!(
        charlie_instance,
        remove_member(group_uuid: group_uuid, user_uuid: alice_uuid)
    )
    .unwrap();

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let bob_location = (32.0853, 34.7818);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(bob_instance, send_location(group_uuid: group_uuid, longitude: bob_location.0, latitude: bob_location.1, timestamp: now)).unwrap();

    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(charlie_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 2);

    // bob adds alice back

    call!(
        bob_instance,
        add_member(group_uuid: group_uuid, user_uuid: alice_uuid)
    )
    .unwrap();

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let bob_location = (32.0853, 34.7818);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    call!(bob_instance, send_location(group_uuid: group_uuid, longitude: bob_location.0, latitude: bob_location.1, timestamp: now)).unwrap();

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let num_locations = call!(alice_instance, get_num_location(group_uuid: group_uuid, client: bob_client, from_timestamp: 0, to_timestamp: future) -> Result<i64, ()>).unwrap();
    assert_eq!(num_locations, 2);

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let num_members = groups[0].members.len();
    assert_eq!(num_members, 3);

    call!(bob_instance, leave_group(group_uuid: group_uuid)).unwrap();

    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let num_members = groups[0].members.len();
    assert_eq!(num_members, 2);

    let groups = call!(charlie_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let num_members = groups[0].members.len();
    assert_eq!(num_members, 2);

    call!(alice_instance, leave_group(group_uuid: group_uuid)).unwrap();

    // alice sends to charlie

    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();
    call!(charlie_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(charlie_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let num_members = groups[0].members.len();
    assert_eq!(num_members, 1);

    call!(
        charlie_instance,
        add_member(group_uuid: group_uuid, user_uuid: bob_uuid)
    )
    .unwrap();

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    let num_members = groups[0].members.len();
    assert_eq!(num_members, 2);

    // group status
    let group_name = "test group";

    call!(charlie_instance, update_group(group_uuid: group_uuid, name: Some(group_name)) -> Result<(), ()>).unwrap();

    let groups = call!(charlie_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups[0].name, Some(group_name.to_string()));

    // ensure the update was sent

    call!(bob_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(bob_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups[0].name, Some(group_name.to_string()));

    // now we add alice back to the group

    call!(
        bob_instance,
        add_member(group_uuid: group_uuid, user_uuid: alice_uuid)
    )
    .unwrap();

    // send the status update
    call!(bob_instance, send_group_status(group_uuid: group_uuid) -> Result<(), ()>).unwrap();

    // receive messages
    call!(alice_instance, receive_messages() -> Result<usize, ()>).unwrap();

    let groups = call!(alice_instance, get_groups() -> Result<Vec<Group>, ()>).unwrap();
    assert_eq!(groups[0].name, Some(group_name.to_string()));
}
