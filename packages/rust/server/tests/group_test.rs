use crate::helper::{get_user_group, start_server};
use axum::http::StatusCode;
use bubble::models::group::{Group, Role};

use bubble::routes::group::GroupName;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    // let _clean = cleanup!(|db| {
    //     db.execute("DELETE FROM \"session_token\"").await.unwrap();
    //     db.execute("DELETE FROM \"user_group\"").await.unwrap();
    //     db.execute("DELETE FROM \"group\"").await.unwrap();
    //     db.execute("DELETE FROM \"location_group\"").await.unwrap();
    //     db.execute("DELETE FROM \"user\"").await.unwrap();
    // });
    let first_username: &str = "Jason Yu";
    //test_user is not explicity tested
    let (token, _test_user) = helper::initialize_user(&db, &client, first_username).await;
    let bearer = format!("Bearer {}", token);
    let _res_create = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: "test_group_1".to_string(),
            })
            .unwrap(),
        )
        .header("Authorization", bearer)
        .send()
        .await;

    // //201 is successful http request, 401 is UNAUTHORIZED
    assert_eq!(_res_create.status(), StatusCode::CREATED);

    let group = Group::from_id(&db, 1)
        .await
        .expect("No group exists in database.");
    assert_eq!(group.id, 1);
    assert_eq!(group.group_name, "test_group_1");

    let (group_id, role_id) = get_user_group(&db, 1).await.unwrap();
    assert_eq!(group_id, 1);
    assert_eq!(role_id, Role::Admin as i32);

    //Session Token is not tested explicity as it is
    // necessary for the above tests to function and it is out of scope.

    let second_username: &str = "Joshua Brown";
    let (token_2, _test_user_2) = helper::initialize_user(&db, &client, second_username).await;
    let bearer_2 = format!("Bearer {}", token_2);
    let _res_create_2 = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: "test_group_2".to_string(),
            })
            .unwrap(),
        )
        .header("Authorization", bearer_2)
        .send()
        .await;

    assert_eq!(_res_create_2.status(), StatusCode::CREATED);

    let group_2 = Group::from_id(&db, 2)
        .await
        .expect("No group exists in database.");
    assert_eq!(group_2.id, 2);
    assert_eq!(group_2.group_name, "test_group_2");

    let (group_id_2, role_id_2) = get_user_group(&db, 2).await.unwrap();
    assert_eq!(group_id_2, 2);
    assert_eq!(role_id_2, Role::Admin as i32);
}
//
// #[tokio::test]
// async fn read_group() {
//     let (db, client) = start_server().await;
//
//     let _clean = cleanup!(|db| {
//          db.execute("DELETE FROM \"session_token\"").await.unwrap();
//         db.execute("DELETE FROM \"group\"").await.unwrap();
//         db.execute("DELETE FROM \"location_group\"").await.unwrap();
//         db.execute("DELETE FROM \"user\"").await.unwrap();
//         db.execute("DELETE FROM \"user_group\"").await.unwrap();
//     });
//
//     let (token, _test_user) = helper::initialize_user(&db, &client).await;
// }
