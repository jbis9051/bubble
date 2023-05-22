use crate::helper::{start_server, TempDatabase};

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
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.json::<MessagesReturned>().await.messages.len(), 0);

    let message = MessageRequest {
        client_uuids: vec![client.uuid.to_string()],
        message: "test message".to_string().into_bytes(),
    };

    let res = server
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let res = server
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let ret = res.json::<MessagesReturned>().await.messages;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0], message.message);
    //
}
#[tokio::test]
async fn negative_test_message() {
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
    let bearer = format!("Bearer {}", token);
    assert!(client.create(db.pool()).await.is_ok());

    // //not a Uuid
    let message = MessageRequest {
        client_uuids: vec![69.to_string()],
        message: "test message".to_string().into_bytes(),
    };

    let res = server
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    //not an existing Uuid
    let message = MessageRequest {
        client_uuids: vec![Uuid::new_v4().to_string()],
        message: "test message".to_string().into_bytes(),
    };

    let res = server
        .post("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    //the client does not exist

    let request_messages = CheckMessages {
        client_uuid: Uuid::new_v4().to_string(),
    };

    let res = server
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    //the client belongs to a different user
    let created_user2 = CreateUser {
        email: "test2@gmail.com".to_string(),
        username: "test_username2".to_string(),
        password: "test_password2".to_string(),
        name: "test_name2".to_string(),
    };
    let (_token2, user2) = helper::initialize_user(db.pool(), &server, &created_user2)
        .await
        .unwrap();

    let mut client2 = Client {
        id: 0,
        user_id: user2.id,
        uuid: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    assert!(client2.create(db.pool()).await.is_ok());

    let request_messages = CheckMessages {
        client_uuid: client2.uuid.to_string(),
    };

    //first user's token is used here
    let res = server
        .get("/message")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_messages).unwrap())
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}
