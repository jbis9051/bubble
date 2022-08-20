use crate::helper::{start_server, TempDatabase};
use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;
use bubble::models::user::User;
use bubble::routes::user::{
    ChangeEmail, Confirm, CreateUser, DeleteJson, Email, ForgotConfirm, SessionToken, SignInJson,
};
use std::borrow::Borrow;


use bubble::models::forgot::Forgot;
use uuid::Uuid;

use bubble::models::session::Session;

mod helper;

#[tokio::test]
async fn create_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test".to_string(),
        password: "password".to_string(),
        phone: Some("18001239876".to_string()),
        name: "John Doe".to_string(),
    };

    let res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&created_user).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let user = User::from_username(db.pool(), &created_user.username)
        .await
        .unwrap();

    assert_eq!(User::verify_password(&user, "password"), true);
    assert_eq!(user.username, created_user.username);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);
    assert_eq!(user.deleted, None);

    let confirmations = Confirmation::filter_user_id(db.pool(), user.id)
        .await
        .unwrap();
    assert_eq!(confirmations.len(), 1);

    let confirmation = &confirmations[0];

    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.email, created_user.email);

    let confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: confirmation.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(confirm_res.status(), StatusCode::CREATED);

    let user = User::from_username(db.pool(), &created_user.username)
        .await
        .unwrap();

    assert_eq!(User::verify_password(&user, "password"), true);
    assert_eq!(user.username, created_user.username);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some(created_user.email));
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);
    assert_eq!(user.deleted, None);
}

#[tokio::test]
async fn test_signin_signout() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };

    let (token, mut user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.token, token);

    helper::signout_user(db.pool(), &client, &session)
        .await
        .unwrap();

    let sessions = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(sessions.len(), 0);

    user.password = Some("testpassword".to_string());
    let token = helper::signin_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);
}

#[tokio::test]
async fn test_forgot_password() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);

    let email_in = Email {
        email: "test@gmail.com".to_string(),
    };
    let res = client
        .post("/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email_in).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let forgot: Forgot = sqlx::query("SELECT * FROM forgot_password WHERE user_id = $1;")
        .bind(user.id)
        .fetch_one(db.pool())
        .await
        .unwrap()
        .borrow()
        .into();

    assert_eq!(forgot.user_id, user.id);
    let confirm = ForgotConfirm {
        password: "newtestpassword".to_string(),
        forgot_code: forgot.forgot_id.to_string(),
    };

    let res = client
        .post("/user/forgot-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_id(db.pool(), user.id).await.unwrap();

    assert_eq!(User::verify_password(&user, "newtestpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
}

#[tokio::test]
async fn test_change_email() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "emailtest@gmail.com".to_string(),
        username: "emailtestusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "emailtestusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("emailtest@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);

    let change = ChangeEmail {
        new_email: "newtest@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let link_id = helper::change_email(db.pool(), &client, &change, &session)
        .await
        .unwrap();
    let confirmation = Confirmation::from_link_id(db.pool(), &link_id)
        .await
        .unwrap();
    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.link_id, link_id);
    assert_eq!(confirmation.email, change.new_email);

    let confirm = Confirm {
        link_id: link_id.to_string(),
    };
    let (user, token) = helper::change_email_confirm(db.pool(), &client, &confirm)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "emailtestusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("newtest@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);
}

#[tokio::test]
async fn test_delete_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);

    let delete_in = DeleteJson {
        password: "testpassword".to_string(),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .delete("/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete_in).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_id(db.pool(), user.id).await.unwrap();

    assert_eq!(user.password, None);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "Deleted Account".to_string());
    assert_eq!(user.deleted.is_some(), true);

    let vec = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(vec.len(), 0);
}

#[tokio::test]
async fn test_negative_user_signup() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (user, link_id) = helper::signup_user(db.pool(), &client, &user)
        .await
        .unwrap();

    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);

    let confirmation = Confirmation::from_link_id(db.pool(), &link_id)
        .await
        .unwrap();
    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.link_id, link_id);
    assert_eq!("test@gmail.com".to_string(), confirmation.email);

    let test = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(test.len(), 0);

    let signin = SignInJson {
        email: "test@gmail".to_string(),
        password: "testpassword".to_string(),
    };
    let test = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(test.status(), StatusCode::UNAUTHORIZED);

    let email_in = Email {
        email: "test@gmail.com".to_string(),
    };
    let test = client
        .post("/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email_in).unwrap())
        .send()
        .await;
    assert_eq!(test.status(), StatusCode::NOT_FOUND);

    let confirm = Confirm {
        link_id: link_id.to_string(),
    };
    let (user, token) = helper::signup_confirm_user(db.pool(), &client, &confirm, &user)
        .await
        .unwrap();
    assert_eq!(User::verify_password(&user, "testpassword"), true);
    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);

    let test = Confirmation::filter_user_id(db.pool(), user.id)
        .await
        .unwrap();
    assert_eq!(test.len(), 0);

    let session = Session::from_token(db.pool(), &token).await.unwrap();
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.token, token);
}

#[tokio::test]
async fn test_negative_signin_signout() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };

    let (token_1, mut user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    user.password = Some("testpassword".to_string());
    let session_1 = Session::from_token(db.pool(), &token_1).await.unwrap();
    assert_eq!(session_1.user_id, user.id);
    assert_eq!(session_1.token, token_1);

    let token_2 = helper::signin_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session_2 = Session::from_token(db.pool(), &token_2).await.unwrap();
    assert_eq!(session_2.user_id, user.id);
    assert_eq!(session_2.token, token_2);

    let signin = SignInJson {
        email: "test@gmail.com".to_string(),
        password: "faketestpassword".to_string(),
    };
    let test = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(test.status(), StatusCode::UNAUTHORIZED);

    let signin = SignInJson {
        email: "faketest@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let test = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(test.status(), StatusCode::UNAUTHORIZED);

    let token = SessionToken {
        token: session_1.token.to_string(),
    };
    let bearer = format!("Bearer {}", token.token);
    let res = client
        .delete("/user/signout")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&token).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let test = Session::from_token(db.pool(), &session_1.token).await;
    assert_eq!(test.is_err(), true);

    let token_3 = helper::signin_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session_3 = Session::from_token(db.pool(), &token_3).await.unwrap();
    assert_eq!(session_3.user_id, user.id);
    assert_eq!(session_3.token, token_3);

    let token = SessionToken {
        token: session_3.token.to_string(),
    };
    let bearer = format!("Bearer {}", token.token);
    let res = client
        .delete("/user/signout")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&token).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let test = Session::from_token(db.pool(), &session_3.token).await;
    assert_eq!(test.is_err(), true);

    let token = SessionToken {
        token: session_3.token.to_string(),
    };
    let res = client
        .delete("/user/signout")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&token).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_negative_forgot() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };

    let (token, mut user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    user.password = Some("testpassword".to_string());

    let email = Email {
        email: user.email.clone().unwrap(),
    };
    let res = client
        .post("/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let res = client
        .post("/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let res = client
        .post("/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let forgots = Forgot::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(forgots.len(), 3);
    let forgot = &forgots[2];

    let session = Session::from_token(db.pool(), &token).await.unwrap();
    assert_eq!(session.user_id, user.id);

    let confirm = ForgotConfirm {
        password: "newtestpassword".to_string(),
        forgot_code: forgot.forgot_id.to_string(),
    };
    let res = client
        .post("/user/forgot-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_id(db.pool(), user.id).await.unwrap();
    assert_eq!(User::verify_password(&user, "newtestpassword"), true);
    let vec = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(vec.len(), 0);

    let signin = SignInJson {
        email: user.email.clone().unwrap(),
        password: "testpassword".to_string(),
    };
    let res = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let signin = SignInJson {
        email: user.email.unwrap(),
        password: "newtestpassword".to_string(),
    };
    let res = client
        .post("/user/signin")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&signin).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let session = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(session.len(), 1);
}

#[tokio::test]
async fn test_negative_change_email() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();

    let change = ChangeEmail {
        new_email: "newtestemail@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let bearer = format!("Bearer {}", "37aa15e0-8a5f-4f75-8c95-bb1238755187");
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let change = ChangeEmail {
        password: "testpassword".to_string(),
        new_email: "newtestemail@gmail.com".to_string(),
    };
    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let change = ChangeEmail {
        new_email: "difftestemail@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let change = ChangeEmail {
        new_email: "difftestemail@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let bearer = format!("Bearer {}", "bad_token");
    let res = client
        .post("/user/email")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&change).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let confirmations = Confirmation::filter_user_id(db.pool(), user.id)
        .await
        .unwrap();
    assert_eq!(confirmations.len(), 4);

    let bad_confirm = Confirm {
        link_id: "incorrect".to_string(),
    };
    let res = client
        .post("/user/email-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&bad_confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let bad_confirm = Confirm {
        link_id: "9719ce93-4023-45ad-8d0b-dac9e48e04b8".to_string(),
    };
    let res = client
        .post("/user/email-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&bad_confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let confirm = Confirm {
        link_id: confirmations[3].link_id.to_string(),
    };
    let res = client
        .post("/user/email-confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let token: SessionToken = res.json().await;

    let confirmations = Confirmation::filter_user_id(db.pool(), user.id)
        .await
        .unwrap();
    assert_eq!(confirmations.len(), 0);

    let sessions = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(sessions.len(), 1);

    let user = User::from_session(db.pool(), Uuid::parse_str(&token.token).unwrap())
        .await
        .unwrap();
    assert_eq!(user.email, Some("difftestemail@gmail.com".to_string()));
}

#[tokio::test]
async fn test_negative_delete() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token, _user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();

    let delete = DeleteJson {
        password: "testpassword".to_string(),
    };
    let bearer = format!("Bearer {}", "bad_token");
    let res = client
        .delete("/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let delete = DeleteJson {
        password: "testpassword".to_string(),
    };
    let bearer = format!("Bearer {}", "37aa15e0-8a5f-4f75-8c95-bb1238755187");
    let res = client
        .delete("/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let delete = DeleteJson {
        password: "incorrect_password".to_string(),
    };
    let bearer = format!("Bearer {}", token);
    let res = client
        .delete("/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
