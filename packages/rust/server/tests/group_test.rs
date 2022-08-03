use crate::helper::{get_user_group, start_server};
use axum::http::StatusCode;
use std::borrow::Borrow;

use bubble::models::group::{Group, Role};
use bubble::routes::group::GroupInfo;
use sqlx::Executor;

use uuid::Uuid;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let _clean = cleanup!(|db| {
        db.execute("DELETE FROM \"session_token\"").await.unwrap();
        db.execute("DELETE FROM \"user_group\"").await.unwrap();
        db.execute("DELETE FROM \"group\"").await.unwrap();
        db.execute("DELETE FROM \"location_group\"").await.unwrap();
        db.execute("DELETE FROM \"user\"").await.unwrap();
    });

    //Test: Creating Group 1

    let first_username: &str = "Jason Yu";
    //test_user is not explicity tested
    let (token, _test_user) = helper::initialize_user(&db, &client, first_username).await;
    let bearer = format!("Bearer {}", token);
    let res = helper::create_group(&db, &client, "test_group_1", bearer)
        .await
        .unwrap();
    let status = res.status();
    let group_info: GroupInfo = res.json().await;

    assert_eq!(status, StatusCode::CREATED);

    let group_uuid = group_info.uuid;
    let group_name = group_info.name;

    assert_eq!(group_name, "test_group_1");

    let group = Group::from_uuid(&db, Uuid::parse_str(&group_uuid).unwrap())
        .await
        .expect("No group exists in database.");
    assert_eq!(group.group_name, "test_group_1");
    // //201 is successful http request, 401 is UNAUTHORIZED

    let role_id = get_user_group(&db, group.id).await.unwrap();
    assert_eq!(role_id, Role::Admin as i32);

    //Session Token is not tested explicity as it is
    //necessary for the above tests to function and it is out of scope.

    //Test: Create Group 2

    let second_username: &str = "Joshua Brown";
    //test_user is not used here. However, it may be used in user_tests AND in future group_tests
    let (token, _test_user) = helper::initialize_user(&db, &client, second_username).await;
    let bearer = format!("Bearer {}", token);
    let res = helper::create_group(&db, &client, "test_group_2", bearer)
        .await
        .unwrap();
    let status = res.status();
    let group_info: GroupInfo = res.json().await;

    assert_eq!(status, StatusCode::CREATED);

    let group_uuid = group_info.uuid;
    let group_name = group_info.name;

    assert_eq!(group_name, "test_group_2");

    let group = Group::from_uuid(&db, Uuid::parse_str(&group_uuid).unwrap())
        .await
        .expect("No group exists in database.");
    assert_eq!(group.group_name, "test_group_2");
    // //201 is successful http request, 401 is UNAUTHORIZED

    let role_id = get_user_group(&db, group.id).await.unwrap();
    assert_eq!(role_id, Role::Admin as i32);
}

#[tokio::test]
async fn read_group() {
    let (db, client) = start_server().await;

    let _clean = cleanup!(|db| {
        db.execute("DELETE FROM \"session_token\"").await.unwrap();
        db.execute("DELETE FROM \"user_group\"").await.unwrap();
        db.execute("DELETE FROM \"group\"").await.unwrap();
        db.execute("DELETE FROM \"location_group\"").await.unwrap();
        db.execute("DELETE FROM \"user\"").await.unwrap();
    });

    let first_username: &str = "Gannon Smith";

    let (token, _test_user) = helper::initialize_user(&db, &client, first_username).await;
    let bearer = format!("Bearer {}", token);
    let res = helper::create_group(&db, &client, "test_group_1", bearer)
        .await
        .unwrap();

    let _status = res.status();
    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_name = group_info.name;
    let group_uuid = group_info.uuid;

    let read_route = format!("/group/{}", group_uuid);

    //res is now for read api route
    let bearer = format!("Bearer {}", token);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    let read_group: GroupInfo = res.json().await;

    assert_eq!(read_group.name, group_name);
    assert_eq!(read_group.uuid, group_uuid);
}
