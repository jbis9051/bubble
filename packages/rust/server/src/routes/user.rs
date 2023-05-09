use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use axum::{Extension, Json};

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::confirmation::Confirmation;

use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::forgot::Forgot;
use crate::models::session::Session;
use crate::models::user::User;
use crate::routes::map_sqlx_err;
use crate::services::email::EmailService;
use crate::services::password;
use crate::services::session::create_session;
use crate::types::DbPool;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", delete(delete_user))
        .route("/register", post(register))
        .route("/confirm", patch(confirm))
        .route("/session", post(login).delete(logout))
        .route("/forgot", post(forgot))
        .route("/reset", get(reset_check).patch(reset))
        .route("/email", post(change_email))
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub name: String,
}

async fn register(
    db: Extension<DbPool>,
    email_service: Extension<EmailService>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    // so technically there is race condition here, but I'm too lazy to avoid it

    if (User::from_username(&db.0, &payload.username).await).is_ok() {
        return Err((StatusCode::CONFLICT, "username".to_string()));
    }
    if (User::from_email(&db.0, &payload.email).await).is_ok() {
        return Err((StatusCode::CONFLICT, "email".to_string()));
    }

    let hash = password::hash(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to hash password".to_string(),
        )
    })?;

    let mut user = User {
        id: 0,
        uuid: Uuid::new_v4(),
        username: payload.username,
        password: hash,
        email: None,
        name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    user.create(&db.0).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to create user".to_string(),
        )
    })?;

    let mut confirmation = Confirmation {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        email: payload.email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    confirmation.create(&db.0).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to create confirmation".to_string(),
        )
    })?;

    email_service
        .send(&format!(
            "to: {} | body: token: {}",
            confirmation.email, confirmation.token
        ))
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to send email".to_string(),
            )
        })?;

    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize)]
pub struct Confirm {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionToken {
    pub token: String,
}

async fn confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let confirmation = Confirmation::from_token(
        &db.0,
        &Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?,
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    confirmation.delete(&db.0).await.map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db.0, confirmation.user_id)
        .await
        .map_err(map_sqlx_err)?;
    user.email = Some(confirmation.email);
    user.update(&db.0).await.map_err(map_sqlx_err)?;

    // whenever we change a user's email we should invalidate all tokens

    Session::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;

    let token = create_session(&db.0, user.id).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::OK,
        Json(SessionToken {
            token: token.to_string(),
        }),
    ))
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

async fn login(
    db: Extension<DbPool>,
    Json(payload): Json<Login>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let user = User::from_email(&db.0, &payload.email)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !password::verify(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_session(&db.0, user.id).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: token.to_string(),
        }),
    ))
}

async fn logout(
    db: Extension<DbPool>,
    Json(payload): Json<SessionToken>,
    _user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    // so while with authenticate the user, this permits any user to delete any session token
    // formally, this could be considered a problem (users should only be able to delete *their* session tokens),
    // but in reality if you have another users session token, you could repeat this request with that token
    // and delete their session token anyway, so it's not really a problem

    let token = Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let session = Session::from_token(&db.0, &token)
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
    email_service: Extension<EmailService>,
    Json(payload): Json<Email>,
) -> Result<StatusCode, StatusCode> {
    let user = User::from_email(&db.0, &payload.email)
        .await
        .map_err(|_| StatusCode::CREATED)?;

    let mut forgot = Forgot {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    forgot.create(&db.0).await.map_err(map_sqlx_err)?;

    email_service
        .send(&format!(
            "to: {} | body: token: {}",
            payload.email, forgot.token
        ))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
    pub password: String,
    pub token: String,
}

async fn reset(
    db: Extension<DbPool>,
    Json(payload): Json<PasswordReset>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    let forgot = Forgot::from_token(&db.0, &uuid)
        .await
        .map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db.0, forgot.user_id)
        .await
        .map_err(map_sqlx_err)?;

    // invalidate all forgot's and session tokens when a user successfully resets their password

    Forgot::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?; // TODO: consider account hijacking attacks

    Session::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;

    user.password =
        password::hash(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user.update(&db.0).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct PasswordResetCheck {
    pub token: String,
}

async fn reset_check(
    db: Extension<DbPool>,
    Query(payload): Query<PasswordResetCheck>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&payload.token).map_err(|_| StatusCode::BAD_REQUEST)?;
    if Forgot::from_token(&db.0, &uuid).await.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct ChangeEmail {
    pub new_email: String,
    pub password: String,
}

async fn change_email(
    db: Extension<DbPool>,
    email_service: Extension<EmailService>,
    Json(payload): Json<ChangeEmail>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if !password::verify(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut change = Confirmation {
        id: 0,
        user_id: user.0.id,
        token: Uuid::new_v4(),
        email: payload.new_email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    change.create(&db.0).await.map_err(map_sqlx_err)?;

    email_service
        .send(&format!(
            "to: {} | body: token: {}",
            change.email, change.token
        ))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize)]
pub struct Delete {
    pub password: String,
}

async fn delete_user(
    db: Extension<DbPool>,
    Json(payload): Json<Delete>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if !password::verify(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    user.delete(&db.0).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}
