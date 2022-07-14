use crate::helper::start_server;
use axum::http::StatusCode;
use bubble::models::group::Group;
use bubble::routes::group::GroupName;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let res = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: "test_group".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;
    //201 is successful http request
    assert_eq!(res.status(), StatusCode::CREATED);
    let group = Group::get_group_by_id(&db, 1)
        .await
        .expect("No group exists in database.");
    assert_eq!(group.id, 1);
    assert_eq!(group.group_name, "test_group");
    //must eventually check for joined table and resolve issues with
}
