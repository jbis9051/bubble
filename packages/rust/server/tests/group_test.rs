use crate::helper::start_server;
use axum::http::StatusCode;
use std::borrow::Borrow;

use bubble::models::group::{Group, Role};
use bubble::routes::group::{GroupInfo, UserID};
use sqlx::Executor;

use uuid::Uuid;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<i32>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
     }, |db, resources| {
         if let Some((group_id, user_id)) = resources.user_group_id {
            sqlx::query("
            DELETE
            FROM user_group
            WHERE user_id = $1
            AND group_id = $2;")
            .bind(&user_id)
            .bind(&group_id)
            .execute(&db)
            .await
            .unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE id = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_username: &str = "Rina Sawayama";

    let (token, test_user) = helper::initialize_user(&db, &client, first_username).await;
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

    let role_id = group.role(&db, test_user.id).await.unwrap();
    assert_eq!(role_id, Role::Admin);

    let user_group: (i32, i32) = (test_user.id, group.id);

    cleanup.resources.user_group_id = Some(user_group);
    cleanup.resources.group_id = Some(group.id);
    cleanup.resources.session_token = Some(token);
    cleanup.resources.user_id = Some(test_user.id);
}

#[tokio::test]
async fn read_group() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<i32>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
     }, |db, resources| {
         if let Some((group_id, user_id)) = resources.user_group_id {
            sqlx::query("
            DELETE
            FROM user_group
            WHERE user_id = $1
            AND group_id = $2;")
            .bind(&user_id)
            .bind(&group_id)
            .execute(&db)
            .await
            .unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE id = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_username: &str = "Madonna";

    let (token, test_user) = helper::initialize_user(&db, &client, first_username).await;
    let bearer = format!("Bearer {}", token);
    let res = helper::create_group(&db, &client, "test_group_1", bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_name = group_info.name;
    let group_uuid = group_info.uuid;

    let read_route = format!("/group/{}", group_uuid);

    let bearer = format!("Bearer {}", token);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    let read_group: GroupInfo = res.json().await;

    assert_eq!(read_group.name, group_name);
    assert_eq!(read_group.uuid, group_uuid);

    let group_uuid = Uuid::parse_str(&group_uuid).unwrap();
    let temp_group = Group::from_uuid(&db, group_uuid).await.unwrap();
    let user_group: (i32, i32) = (temp_group.id, test_user.id);

    cleanup.resources.session_token = Some(token);
    cleanup.resources.group_id = Some(temp_group.id);
    cleanup.resources.user_id = Some(test_user.id);
    cleanup.resources.user_group_id = Some(user_group);
}

#[tokio::test]
async fn add_user() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<Vec<(i32, i32)>>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<Uuid>,
        pub session_token: Option<Vec<Uuid>>,
        pub user_id: Option<Vec<i32>>
     }, |db, resources| {
         if let Some(user_group_ids) = resources.user_group_id {
            for i in user_group_ids {
            sqlx::query("
            DELETE
            FROM user_group
            WHERE user_id = $1
            AND group_id = $2;")
            .bind(&i.0)
            .bind(&i.1)
            .execute(&db)
            .await
            .unwrap();
                }
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE uuid = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
            for i in session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&i).execute(&db).await.unwrap();
        }
            }
        if let Some(user_id) = resources.user_id {
            for i in user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&i).execute(&db).await.unwrap();
        }
            }
    });

    let first_username: &str = "Porter Robinson";
    let (token_admin, creator) = helper::initialize_user(&db, &client, first_username).await;
    let bearer = format!("Bearer {}", token_admin);
    let res = helper::create_group(&db, &client, "test_group_1", bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;

    let first_username: &str = "Billy Joel";
    let (token_user_1, billy_joel) = helper::initialize_user(&db, &client, first_username).await;

    let first_username: &str = "Kanye West";
    let (token_user_2, kanye_west) = helper::initialize_user(&db, &client, first_username).await;

    let mut user_ids: Vec<String> = Vec::new();
    user_ids.push(billy_joel.uuid.to_string());
    user_ids.push(kanye_west.uuid.to_string());

    let read_route = format!("/group/{}/new_users", group_uuid);

    let bearer = format!("Bearer {}", token_admin);
    println!("HELLLO: {}", bearer);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let group_uuid_ref: &str = &*group_uuid;
    let new_group = Group::from_uuid(&db, Uuid::parse_str(group_uuid_ref).unwrap())
        .await
        .unwrap();
    let billy_joel_role = new_group.role(&db, billy_joel.id).await.unwrap();
    let kanye_west_role = new_group.role(&db, kanye_west.id).await.unwrap();
    assert_eq!(billy_joel_role, Role::Child);
    assert_eq!(kanye_west_role, Role::Child);

    let mut tokens: Vec<Uuid> = Vec::new();
    tokens.push(token_admin);
    tokens.push(token_user_1);
    tokens.push(token_user_2);

    let mut users: Vec<i32> = Vec::new();
    let mut user_groups: Vec<(i32, i32)> = Vec::new();
    user_groups.push((creator.id, new_group.id));
    user_groups.push((billy_joel.id, new_group.id));
    user_groups.push((kanye_west.id, new_group.id));

    users.push(creator.id);
    users.push(billy_joel.id);
    users.push(kanye_west.id);

    let group_uuid_ref: &str = &*group_uuid;

    cleanup.resources.user_group_id = Some(user_groups);
    cleanup.resources.group_id = Some(Uuid::parse_str(group_uuid_ref).unwrap());
    cleanup.resources.session_token = Some(tokens);
    cleanup.resources.user_id = Some(users);
}
