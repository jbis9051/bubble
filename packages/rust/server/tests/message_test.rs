use crate::helper::{start_server, TempDatabase};
use std::borrow::Borrow;

use axum::http::StatusCode;
use bubble::models::client::Client;
use bubble::routes::message::{CheckMessages, MessageRequest, MessagesReturned};
use bubble::routes::user::CreateUser;
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

mod helper;

#[tokio::test]
async fn test_message() {
    let db = TempDatabase::new().await;
    let server = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test_username".to_string(),
        password: "test_password".to_string(),
        name: "test_name".to_string(),
    };
    let (token, user) = helper::initialize_user(db.pool(), &server, &created_user)
        .await
        .unwrap();

    let mut client = Client {
        id: 0,
        user_id: user.id,
        uuid: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    assert!(client.create(db.pool()).await.is_ok());

    let request_messages = CheckMessages {
        client_uuid: client.uuid.to_string(),
    };

    let bearer = format!("Bearer {}", token);
    let res = server
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.json::<MessagesReturned>().await.messages.len(), 0);

    let message = MessageRequest {
        client_uuids: vec![client.uuid.to_string()],
        message: "test message".to_string().into_bytes(),
    };

    let bearer = format!("Bearer {}", token);
    let res = server
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let bearer = format!("Bearer {}", token);
    let res = server
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let ret = res.json::<MessagesReturned>().await.messages;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0], message.message);
}
