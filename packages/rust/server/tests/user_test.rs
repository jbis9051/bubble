use crate::helper::start_server;
use axum::body::Body;
use axum::http::StatusCode;
use bubble::models::user::User;
use bubble::routes::user::CreateUser;
use tokio::time;
use tokio::time::sleep;

mod helper;

#[tokio::test]
async fn create_user() {
    let (db, client) = start_server().await;

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

    /*let user = User::get_by_email(&db, "test1@gmail.com")
        .await
        .expect("user doesn't exist");

    assert_eq!(user.email, "test@gmail.com");
    assert_eq!(user.username, "test");
    assert_eq!(user.password, "password");
    assert_eq!(user.phone, None);*/
}
