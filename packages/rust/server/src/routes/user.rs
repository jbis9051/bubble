use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::Router;
use axum::{Extension, Json};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::confirmation::Confirmation;

use crate::models::forgot::Forgot;
use crate::models::session::Session;
use crate::models::user::User;
use crate::routes::map_sqlx_err;
use crate::types::DbPool;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", delete(delete_user))
        .route("/signup", post(signup))
        .route("/signup-confirm", post(signup_confirm))
        .route("/signin", post(signin))
        .route("/signout", delete(signout))
        .route("/forgot", post(forgot))
        .route("/forgot-confirm", post(forgot_confirm))
        .route("/change-email", post(change_email))
        .route("/change-email-confirm", post(change_email_confirm))
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
    pub name: String,
}

async fn signup(
    db: Extension<DbPool>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, StatusCode> {
    let mut user = User {
        id: 0,
        uuid: Uuid::new_v4(),
        username: payload.username,
        password: String::new(),
        profile_picture: None,
        email: None,
        phone: payload.phone,
        name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
        deleted: None,
    };
    let user = user
        .create(&db.0, &payload.email, &payload.password)
        .await
        .map_err(map_sqlx_err)?;

    let conf = Confirmation {
        id: 0,
        user_id: user.id,
        link_id: Uuid::new_v4(),
        email: payload.email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let conf = conf.create(&db.0).await.map_err(map_sqlx_err)?;

    println!(
        "Sending Email with link_id {:?} to {:?}",
        conf.link_id, conf.email
    );
    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize)]
pub struct Confirm {
    pub link_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionToken {
    pub token: String,
}

async fn signup_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let conf = Confirmation::from_link_id(
        &db.0,
        Uuid::parse_str(&payload.link_id).map_err(|_| StatusCode::BAD_REQUEST)?,
    )
    .await
    .map_err(map_sqlx_err)?;

    conf.delete(&db.0).await.map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db.0, conf.user_id)
        .await
        .map_err(map_sqlx_err)?;

    user.email = Some(conf.email);
    user.update(&db.0).await.map_err(map_sqlx_err)?;

    let session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let session = session.create(&db.0).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
}

#[derive(Serialize, Deserialize)]
pub struct SignInJson {
    pub email: String,
    pub password: String,
}

async fn signin(
    db: Extension<DbPool>,
    Json(payload): Json<SignInJson>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let user = User::from_email(&db.0, &payload.email)
        .await
        .map_err(map_sqlx_err)?;

    let password = payload.password.as_bytes();

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| StatusCode::BAD_REQUEST)?;
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let session = session.create(&db.0).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
}

// user must be signed in
async fn signout(
    db: Extension<DbPool>,
    Json(payload): Json<SessionToken>,
) -> Result<StatusCode, StatusCode> {
    let token = Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let session = Session::from_token(&db.0, token)
        .await
        .map_err(map_sqlx_err)?;
    session.delete(&db.0).await.map_err(map_sqlx_err)?;
    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

async fn forgot(
    db: Extension<DbPool>,
    Json(payload): Json<Email>,
) -> Result<StatusCode, StatusCode> {
    let user = User::from_email(&db.0, &payload.email)
        .await
        .map_err(map_sqlx_err)?;
    let forgot = Forgot {
        id: 0,
        user_id: user.id,
        forgot_id: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let forgot = forgot.create(&db.0).await.map_err(map_sqlx_err)?;

    println!(
        "Sending email with {:?} to {:?}",
        forgot.forgot_id, user.email
    );
    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize)]
pub struct ForgotConfirm {
    pub password: String,
    pub forgot_code: String,
}

async fn forgot_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<ForgotConfirm>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&payload.forgot_code).map_err(|_| StatusCode::BAD_REQUEST)?;
    let forgot = Forgot::from_uuid(&db.0, uuid).await.map_err(map_sqlx_err)?;
    let mut user = User::from_id(&db.0, forgot.user_id)
        .await
        .map_err(map_sqlx_err)?;

    forgot.delete_all(&db.0).await.map_err(map_sqlx_err)?;
    Session::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;
    user.update_password(&db.0, &payload.password)
        .await
        .map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct ChangeEmail {
    pub session_token: String,
    pub new_email: String,
}
//User must be signed in
async fn change_email(
    db: Extension<DbPool>,
    Json(payload): Json<ChangeEmail>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&payload.session_token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let user = User::from_session(&db.0, uuid)
        .await
        .map_err(map_sqlx_err)?;

    let change = Confirmation {
        id: 0,
        user_id: user.id,
        link_id: Uuid::new_v4(),
        email: payload.new_email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let change = change.create(&db.0).await.map_err(map_sqlx_err)?;

    println!("Sending code {:?} to {:?}", change.link_id, change.email);
    Ok(StatusCode::CREATED)
}

async fn change_email_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let confirmation = Confirmation::from_link_id(
        &db.0,
        Uuid::parse_str(&payload.link_id).map_err(|_| StatusCode::BAD_REQUEST)?,
    )
    .await
    .map_err(map_sqlx_err)?;
    let mut user = User::from_id(&db.0, confirmation.user_id)
        .await
        .map_err(map_sqlx_err)?;

    user.email = Some(confirmation.email.clone());
    user.update(&db.0).await.map_err(map_sqlx_err)?;
    confirmation.delete(&db.0).await.map_err(map_sqlx_err)?;

    let sessions = Session::filter_user_id(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;
    for session in sessions {
        session.delete(&db.0).await.map_err(map_sqlx_err)?;
    }

    let session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let session = session.create(&db.0).await.map_err(map_sqlx_err)?;
    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
}

#[derive(Serialize, Deserialize)]
pub struct DeleteJson {
    pub token: String,
    pub password: String,
}
async fn delete_user(
    db: Extension<DbPool>,
    Json(payload): Json<DeleteJson>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut user = User::from_session(&db.0, uuid)
        .await
        .map_err(map_sqlx_err)?;

    let password = payload.password.as_bytes();
    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| StatusCode::BAD_REQUEST)?;
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    user.delete(&db.0).await.map_err(map_sqlx_err)?;
    Ok(StatusCode::OK)
}
