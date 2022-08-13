use crate::helper::{start_server, TempDatabase};
use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;
use bubble::models::user::User;
use bubble::routes::user::{ChangeEmail, Confirm, CreateUser, DeleteJson, Email, ForgotConfirm};
use std::borrow::Borrow;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use bubble::models::forgot::Forgot;

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

    let password = "password".as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(user.username, created_user.username);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);

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

    let password = "password".as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(user.username, created_user.username);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some(created_user.email));
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);
}

#[tokio::test]
async fn create_multiple_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let brian_in = CreateUser {
        email: "python@gmail.com".to_string(),
        username: "machine_learning_man".to_string(),
        password: "lots_of_abstraction".to_string(),
        phone: None,
        name: "Brian".to_string(),
    };
    let (brian, _brian_link) = helper::signup_user(db.pool(), &client, &brian_in)
        .await
        .unwrap();
    let brian_confirmation = &Confirmation::filter_user_id(db.pool(), brian.id)
        .await
        .unwrap()[0];

    let password = "lots_of_abstraction".as_bytes();
    let parsed_hash = PasswordHash::new(&brian.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, None);
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");
    assert_eq!(brian_confirmation.user_id, brian.id);
    assert_eq!(brian_confirmation.email, "python@gmail.com");

    let timmy_in = CreateUser {
        email: "javascript@gmail.com".to_string(),
        username: "web_development_dude".to_string(),
        password: "html_rocks".to_string(),
        phone: Some("66260701534".to_string()),
        name: "Little Timmy III".to_string(),
    };
    let (timmy, _timmy_link) = helper::signup_user(db.pool(), &client, &timmy_in)
        .await
        .unwrap();
    let timmy_confirmation = &Confirmation::filter_user_id(db.pool(), timmy.id)
        .await
        .unwrap()[0];

    let password = "html_rocks".as_bytes();
    let parsed_hash = PasswordHash::new(&timmy.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, None);
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
    assert_eq!(timmy_confirmation.user_id, timmy.id);
    assert_eq!(timmy_confirmation.email, "javascript@gmail.com");

    let brian_confirm_in = Confirm {
        link_id: brian_confirmation.link_id.to_string(),
    };
    let (brian, brian_token) =
        helper::signup_confirm_user(db.pool(), &client, &brian_confirm_in, &brian)
            .await
            .unwrap();
    let brian_session = &Session::filter_user_id(db.pool(), brian.id).await.unwrap()[0];

    let password = "lots_of_abstraction".as_bytes();
    let parsed_hash = PasswordHash::new(&brian.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, Some("python@gmail.com".to_string()));
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");
    assert_eq!(brian_session.user_id, brian.id);
    assert_eq!(brian_session.token, brian_token);

    let bill_in = CreateUser {
        email: "rust@gmail.com".to_string(),
        username: "big_programmer_pro".to_string(),
        password: "cool_crustacean".to_string(),
        phone: Some("18004321234".to_string()),
        name: "bill".to_string(),
    };
    let (bill, _bill_link) = helper::signup_user(db.pool(), &client, &bill_in)
        .await
        .unwrap();
    let bill_confirmation = &Confirmation::filter_user_id(db.pool(), bill.id)
        .await
        .unwrap()[0];

    let password = "cool_crustacean".as_bytes();
    let parsed_hash = PasswordHash::new(&bill.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, None);
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "bill");
    assert_eq!(bill_confirmation.user_id, bill.id);
    assert_eq!(bill_confirmation.email, "rust@gmail.com");

    let bill_confirm_in = Confirm {
        link_id: bill_confirmation.link_id.to_string(),
    };
    let (bill, bill_token) =
        helper::signup_confirm_user(db.pool(), &client, &bill_confirm_in, &bill)
            .await
            .unwrap();
    let bill_session = &Session::filter_user_id(db.pool(), bill.id).await.unwrap()[0];

    let password = "cool_crustacean".as_bytes();
    let parsed_hash = PasswordHash::new(&bill.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, Some("rust@gmail.com".to_string()));
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "bill");
    assert_eq!(bill_session.user_id, bill.id);
    assert_eq!(bill_session.token, bill_token);

    let timmy_confirm_in = Confirm {
        link_id: timmy_confirmation.link_id.to_string(),
    };
    let (timmy, timmy_token) =
        helper::signup_confirm_user(db.pool(), &client, &timmy_confirm_in, &timmy)
            .await
            .unwrap();
    let timmy_session = &Session::filter_user_id(db.pool(), timmy.id).await.unwrap()[0];

    let password = "html_rocks".as_bytes();
    let parsed_hash = PasswordHash::new(&timmy.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, Some("javascript@gmail.com".to_string()));
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
    assert_eq!(timmy_session.user_id, timmy.id);
    assert_eq!(timmy_session.token, timmy_token);
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
    let session = Session::from_token(db.pool(), token).await.unwrap();

    let password = "testpassword".as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.token, token);

    helper::signout_user(db.pool(), &client, &session)
        .await
        .unwrap();

    let sessions = Session::filter_user_id(db.pool(), user.id).await.unwrap();
    assert_eq!(sessions.len(), 0);

    user.password = "testpassword".to_string();
    let token = helper::signin_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), token).await.unwrap();
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
    let (token, mut user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();
    let session = Session::from_token(db.pool(), token).await.unwrap();

    let password = "testpassword".as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
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
    assert_eq!(res.status(), StatusCode::CREATED);

    user.password = "newtestpassword".to_string();
    let _token = helper::signin_user(db.pool(), &client, &user)
        .await
        .unwrap();
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
    let session = Session::from_token(db.pool(), token).await.unwrap();

    let password = "testpassword".as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .unwrap();

    assert_eq!(user.username, "emailtestusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("emailtest@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);

    let change = ChangeEmail {
        session_token: token.to_string(),
        new_email: "newtest@gmail.com".to_string(),
    };
    let link_id = helper::change_email(db.pool(), &client, &change)
        .await
        .unwrap();
    let confirmation = Confirmation::from_link_id(db.pool(), link_id)
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
    println!("token2: {:?}", token);
    let session = Session::from_token(db.pool(), token).await.unwrap();

    assert_eq!(user.username, "emailtestusername");
    println!("password = {}", user.password);
    //assert_eq!(user.password, "testpassword");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("newtest@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
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
    let session = Session::from_token(db.pool(), token).await.unwrap();

    assert_eq!(user.username, "testusername");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(user.deleted, None);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);

    let delete_in = DeleteJson {
        token: token.to_string(),
        password: "testpassword".to_string(),
    };

    let res = client
        .delete("/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete_in).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_id(db.pool(), user.id).await.unwrap();

    assert_eq!(user.password, "".to_string());
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "".to_string());
    assert!(user.deleted.is_some());

    let rows = sqlx::query("SELECT * FROM session WHERE user_id = $1;")
        .bind(user.id)
        .execute(db.pool())
        .await
        .unwrap();
    assert_eq!(rows.rows_affected(), 0);
}
