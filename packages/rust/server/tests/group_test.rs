use crate::helper::{start_server, Cleanup};

use axum::http::StatusCode;

use bubble::models::group::Group;

use bubble::routes::group::GroupName;

use sqlx::Executor;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let (token, _test_user) = helper::initialize_user(&db, &client).await;

    let _clean = cleanup!(|db| {
        db.execute("DELETE FROM \"group\"").await.unwrap();
        db.execute("DELETE FROM \"location_group\"").await.unwrap();
        db.execute("DELETE FROM \"user\"").await.unwrap();
        db.execute("DELETE FROM \"user_group\"").await.unwrap();
    });
    let mut bearer = "Bearer ".to_owned();
    bearer.push_str(&*token);
    let res_group = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: "test_group".to_string(),
            })
            .unwrap(),
        )
        .header("Authorization", bearer)
        .send()
        .await;
    //201 is successful http request
    assert_eq!(res_group.status(), StatusCode::CREATED);
    println!("Reaches up to asserts");
    let group = Group::from_id(&db, 1)
        .await
        .expect("No group exists in database.");
    assert_eq!(group.id, 1);
    assert_eq!(group.group_name, "test_group");
}
