use crate::helper::start_server;
use axum::body::Body;
use axum::http::StatusCode;
use bubble::models::user::User;
use bubble::routes::user::{Confirm, Confirmation, CreateUser};
use sqlx::postgres::PgRow;
use sqlx::postgres::PgSeverity::Error;
use sqlx::Row;
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
                phone: Some("18001239876".to_string()),
                name: "John Doe".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    // TODO

    let user = User::get_by_id(&db, 1).await.expect("user doesn't exist");

    assert_eq!(user.id, 1);
    assert_eq!(user.username, "test");
    assert_eq!(user.password, "password");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, Some("18001239876".to_string()));
    assert_eq!(user.name, "John Doe");

    let row = sqlx::query("SELECT * FROM confirmation WHERE id = 1;")
        .fetch_one(&db)
        .await
        .unwrap();

    let conf = Confirmation {
        id: row.get("id"),
        user_id: row.get("user_id"),
        link_id: row.get("link_id"),
        email: row.get("email"),
        created: row.get("created"),
    };

    assert_eq!(conf.id, 1);
    assert_eq!(conf.user_id, 1);
    assert_eq!(conf.email, "test@gmail.com");

    let confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: conf.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(confirm_res.status(), StatusCode::CREATED);

    let user = User::get_by_id(&db, 1).await.unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.username, "test");
    assert_eq!(user.password, "password");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, Some("18001239876".to_string()));
    assert_eq!(user.name, "John Doe");
}
