use crate::helper::start_server;
use axum::body::Body;
use axum::http::StatusCode;
use bubble::routes::user::CreateUser;
use tokio::time;
use tokio::time::sleep;

mod helper;

#[tokio::test]
async fn create_user() {
    let client = start_server().await;

    let res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateUser {
                email: "test@gmail.com".to_string(),
                username: "test".to_string(),
                password: "password".to_string(),
                phone: None,
                name: "Test McTestin".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    // TODO
}
