use crate::helper::start_server;
use axum::http::StatusCode;
use bubble::models::group::Group;


mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let res = client
        .post("/group/test_group")
        .header("Content-Type", "application/json")
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let group = Group::get_group_by_id(&db, 1)
        .await
        .expect("No group exists in database.");
    assert_eq!(group.id, 1);
    assert_eq!(group.group_name, "test_group");
    //must eventually check for joined table and resolve issues with
}
