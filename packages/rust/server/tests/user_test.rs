use crate::crypto_helper::{generate_ed25519_keypair, PRIVATE, PUBLIC};
use crate::helper::{create_client, start_server, TempDatabase};
use axum::http::StatusCode;
use common::base64::Base64;
use common::http_types::{
    ChangeEmail, ConfirmEmail, CreateUser, DeleteUser, ForgotEmail, Login, PasswordReset,
    PublicUser, UpdateIdentity, UserProfile,
};
use ed25519_dalek::PublicKey;
use server::models::confirmation::Confirmation;
use server::models::forgot::Forgot;
use server::models::session::Session;
use server::models::user::User;
use server::services::password;
use uuid::Uuid;

mod crypto_helper;
mod helper;

#[tokio::test]
async fn test_register() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test".to_string(),
        password: "password".to_string(),
        name: "John Doe".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let res = client
        .post("/v1/user/register")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&created_user).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    assert!(Uuid::parse_str(&res.text().await).is_ok());

    let user = User::from_username(db.pool(), &created_user.username)
        .await
        .unwrap();

    assert!(password::verify(&user.password, &created_user.password).unwrap());
    assert_eq!(user.username, created_user.username);
    assert_eq!(user.email, None);
    assert_eq!(user.name, created_user.name);

    let confirmations = Confirmation::filter_user_id(db.pool(), user.id)
        .await
        .unwrap();
    assert_eq!(confirmations.len(), 1);

    let confirmation = &confirmations[0];

    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.email, created_user.email);

    let confirm_res = client
        .patch("/v1/user/confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&ConfirmEmail {
                token: confirmation.token,
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(confirm_res.status(), StatusCode::OK);

    let user = User::from_username(db.pool(), &created_user.username)
        .await
        .unwrap();

    assert!(password::verify(&user.password, &created_user.password).unwrap());
    assert_eq!(user.username, created_user.username);
    assert_eq!(user.email, Some(created_user.email));
    assert_eq!(user.name, created_user.name);
}

#[tokio::test]
async fn test_login_logout() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert!(password::verify(&user.password, &created_user.password).unwrap());
    assert_eq!(user.username, created_user.username);
    assert_eq!(user.email, Some(created_user.email));
    assert_eq!(user.name, created_user.name);

    assert_eq!(session.user_id, user.id);
    assert_eq!(session.token, token);

    helper::logout(db.pool(), &client, &session).await.unwrap();

    let sessions = Session::filter_user_id(db.pool(), user.id).await.unwrap();

    assert_eq!(sessions.len(), 0);

    let token = helper::login(
        db.pool(),
        &client,
        &Login {
            email: user.email.unwrap().clone(),
            password: created_user.password.clone(),
        },
    )
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
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let (token, user) = helper::initialize_user(db.pool(), &client, &user)
        .await
        .unwrap();

    // ensure a session exists for the user
    assert!(Session::from_token(db.pool(), &token).await.is_ok());

    let email_in = ForgotEmail {
        email: user.email.unwrap(),
    };

    let res = client
        .post("/v1/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email_in).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let forgots = Forgot::filter_user_id(db.pool(), user.id).await.unwrap();

    assert_eq!(forgots.len(), 1);

    let forgot = &forgots[0];

    assert_eq!(forgot.user_id, user.id);

    let res = client
        .get(&format!("/v1/user/reset?token={}", forgot.token))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email_in).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let confirm = PasswordReset {
        password: "newtestpassword".to_string(),
        token: forgot.token,
    };

    let res = client
        .patch("/v1/user/reset")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get(&format!("/v1/user/reset?token={}", forgot.token))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&email_in).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // ensure the session is deleted
    assert!(Session::from_token(db.pool(), &token).await.is_err());

    let user = User::from_id(db.pool(), user.id).await.unwrap();

    assert!(password::verify(&user.password, &confirm.password).unwrap());
}

#[tokio::test]
async fn test_change_email() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "emailtest@gmail.com".to_string(),
        username: "emailtestusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    // ensure a session exists for the user
    let session = Session::from_token(db.pool(), &token).await.unwrap();

    assert_eq!(user.email, Some(created_user.email));

    let change = ChangeEmail {
        new_email: "newtest@gmail.com".to_string(),
        password: created_user.password.clone(),
    };

    let link_id = helper::change_email(db.pool(), &client, &change, &session)
        .await
        .unwrap();

    let confirmation = Confirmation::from_token(db.pool(), &link_id).await.unwrap();

    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.token, link_id);
    assert_eq!(confirmation.email, change.new_email);

    let confirm = ConfirmEmail { token: link_id };

    let res = client
        .patch("/v1/user/confirm")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&confirm).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    // ensure the session is deleted
    assert!(Session::from_token(db.pool(), &token).await.is_err());

    let user = User::from_id(db.pool(), user.id).await.unwrap();

    assert_eq!(user.email, Some(change.new_email));
}

#[tokio::test]
async fn test_delete_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let email = user.email.unwrap();

    assert!(User::from_email(db.pool(), &email).await.is_ok());

    let delete_in = DeleteUser {
        password: created_user.password.clone(),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .delete("/v1/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete_in).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    assert!(User::from_email(db.pool(), &email).await.is_err());
}

#[tokio::test]
async fn test_get_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let res = client
        .get(&format!("/v1/user/{}", user.uuid))
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: PublicUser = res.json().await;

    assert_eq!(payload.uuid, user.uuid);
    assert_eq!(payload.username, user.username);
    assert_eq!(payload.name, user.name);
    assert_eq!(payload.identity.0, user.identity);
    assert_eq!(payload.primary_client_uuid, None);
}

#[tokio::test]
async fn test_replace_identity() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let keypair = generate_ed25519_keypair();
    let public_2 = keypair.public.as_bytes().to_vec();
    let update_identity = UpdateIdentity {
        identity: Base64(public_2.clone()),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .put("/v1/user/identity")
        .json(&update_identity)
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let user = User::from_uuid(db.pool(), &user.uuid).await.unwrap();

    assert_eq!(user.identity, public_2);
}

#[tokio::test]
async fn test_update_profile() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let res = client
        .get(&format!("/v1/user/{}", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: PublicUser = res.json().await;

    assert_eq!(payload.name, created_user.name);
    assert_eq!(payload.primary_client_uuid, None);

    let (_, client_uuid) = create_client(PUBLIC, PRIVATE, &bearer, &client).await;

    let update_profile = UserProfile {
        name: "testname2".to_string(),
        primary_client_uuid: Some(client_uuid),
    };

    let res = client
        .put("/v1/user/profile")
        .json(&update_profile)
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get(&format!("/v1/user/{}", user.uuid))
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let payload: PublicUser = res.json().await;

    assert_eq!(payload.name, update_profile.name);
    assert_eq!(
        payload.primary_client_uuid,
        update_profile.primary_client_uuid
    );
}

// negative tests

#[tokio::test]
async fn test_register_bad_identity() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let identity = vec![0];

    assert!(PublicKey::from_bytes(&identity).is_err());

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(identity),
    };

    let res = client
        .post("/v1/user/register")
        .header("Content-Type", "application/json")
        .json(&created_user)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_register_conflict() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let mut register = CreateUser {
        email: created_user.email,
        username: "testusername2".to_string(),
        password: "testpassword2".to_string(),
        name: "testname2".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let res = client
        .post("/v1/user/register")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&register).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);

    register.email = "test2@gmail.com".to_string();
    register.username = created_user.username;

    let res = client
        .post("/v1/user/register")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&register).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_login_bad_credentials() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let mut login = Login {
        email: created_user.email,
        password: "badpassword".to_string(),
    };

    let res = client
        .post("/v1/user/session")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    login.email = "bad@gmail.com".to_string();

    let res = client
        .post("/v1/user/session")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_forgot_bad_email() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let forgot = ForgotEmail {
        email: "forgot@gmail.com".to_string(),
    };

    let res = client
        .post("/v1/user/forgot")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&forgot).unwrap())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_delete_bad_password() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, _) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let delete_user = DeleteUser {
        password: "badpassword".to_string(),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .delete("/v1/user")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&delete_user).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_change_email_bad_password() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "emailtest@gmail.com".to_string(),
        username: "emailtestusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };

    let (token, _user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let change = ChangeEmail {
        new_email: "newtest@gmail.com".to_string(),
        password: "badpassword".to_string(),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/v1/user/email")
        .json(&change)
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_replace_identity_bad_identity() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, _) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let update_identity = UpdateIdentity {
        identity: Base64(vec![0]),
    };

    let bearer = format!("Bearer {}", token);
    let res = client
        .put("/v1/user/identity")
        .json(&update_identity)
        .header("Authorization", bearer)
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_profile_bad_client() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        name: "testname".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token, _user) = helper::initialize_user(db.pool(), &client, &created_user)
        .await
        .unwrap();

    let created_user_2 = CreateUser {
        email: "test2@gmail.com".to_string(),
        username: "testusername2".to_string(),
        password: "testpassword2".to_string(),
        name: "testname2".to_string(),
        identity: Base64(PUBLIC.to_vec()),
    };
    let (token_2, _user_2) = helper::initialize_user(db.pool(), &client, &created_user_2)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let bearer_2 = format!("Bearer {}", token_2);

    let (_, client_uuid) = create_client(PUBLIC, PRIVATE, &bearer_2, &client).await;

    let update_profile = UserProfile {
        name: "testname2".to_string(),
        primary_client_uuid: Some(client_uuid),
    };

    let res = client
        .put("/v1/user/profile")
        .json(&update_profile)
        .header("Authorization", bearer.clone())
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
