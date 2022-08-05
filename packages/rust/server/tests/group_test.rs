use crate::helper::start_server;
use axum::http::StatusCode;

use bubble::models::group::{Group, Role};
use bubble::routes::group::GroupInfo;
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

    let mut group = Group::from_uuid(&db, Uuid::parse_str(&group_uuid).unwrap())
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

//#[tokio::test]
// async fn read_group() {
//     let (db, client) = start_server().await;
//
//     let mut cleanup = cleanup!({
//         //user_group_id accepts the group_id of entry to be deleted
//         pub user_group_id: Option<(Uuid, i32)>,
//         pub location_group_id: Option<i32>,
//         pub group_id: Option<Uuid>,
//         pub session_token_uuid: Option<Uuid>,
//         pub user_id: Option<i32>
//      }, |db, resources| {
//         //user_id and group_id, get from group_uuid
//
//
//         if let Some((group_uuid, user_id)) = resources.user_group_id {
//             sqlx::query(r#"
//             DELETE "user_group"
//             FROM "user_group"
//             INNER JOIN "group"
//             ON "user_group".group_id = "group".id
//             WHERE "group".uuid = $1
//             AND "user_group".user_id = $2"#)
//             .bind(&group_uuid)
//             .bind(&user_id)
//             .execute(&db)
//             .await
//             .unwrap();
//         }
//         if let Some(location_group_id) = resources.location_group_id {
//                 sqlx::query("DELETE FROM \"location_group\" WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
//         }
//         if let Some(group_id) = resources.group_id {
//                 sqlx::query("DELETE FROM \"group\" WHERE uuid = $1").bind(&group_id).execute(&db).await.unwrap();
//         }
//         if let Some(session_token_uuid) = resources.session_token_uuid {
//                 sqlx::query("DELETE FROM \"session_token\" WHERE uuid = $1").bind(&session_token_uuid).execute(&db).await.unwrap();
//         }
//         if let Some(user_id) = resources.user_id {
//                 sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
//         }
//     });
//
//     let first_username: &str = "Madonna";
//
//     let (token, _test_user) = helper::initialize_user(&db, &client, first_username).await;
//     let bearer = format!("Bearer {}", token);
//     let res = helper::create_group(&db, &client, "test_group_1", bearer)
//         .await
//         .unwrap();
//
//     let _status = res.status();
//     assert_eq!(res.status(), StatusCode::CREATED);
//
//     let group_info: GroupInfo = res.json().await;
//     let group_name = group_info.name;
//     let group_uuid = group_info.uuid;
//
//     let read_route = format!("/group/{}", group_uuid);
//
//     let bearer = format!("Bearer {}", token);
//     let res = client
//         .get(read_route.borrow())
//         .header("Authorization", bearer)
//         .send()
//         .await;
//     let read_group: GroupInfo = res.json().await;
//
//     assert_eq!(read_group.name, group_name);
//     assert_eq!(read_group.uuid, group_uuid);
//
//     let group_uuid = Uuid::parse_str(&group_uuid).unwrap();
//     let user_group:(Uuid, i32) = (group_uuid, _test_user.id);
//
//     cleanup.resources.session_token_uuid = Some(token);
//     cleanup.resources.group_id = Some(group_uuid);
//     cleanup.resources.user_id = Some(_test_user.id);
//     cleanup.resources.user_group_id = Some(user_group);
//
// }
