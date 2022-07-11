use crate::DbPool;

use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use axum::{Extension, Json};

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use serde::{Deserialize, Serialize};

use crate::models::user::User;

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/signup-confirm", post(signup_confirm))
        .route("/signin", post(signin))
    /*
    .route("/signout/:token", post(signout))
    .route("/forgot", post(forgot))
    .route("/forgot-confirm", post(forgot_confirm))
    .route("/change_email", post(change_email))
    .route("", delete(delete_user))*/
}

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    username: String,
    password: String,
    phone: Option<String>,
    name: String,
}
async fn signup(db: Extension<DbPool>, Json(payload): Json<CreateUser>) -> StatusCode {
    let user: User = User {
        id: 0,
        uuid: Uuid::new_v4(),
        username: payload.username,
        password: payload.password,
        profile_picture: None,
        email: None,
        phone: payload.phone,
        name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let user = match User::create(&db.0, user).await {
        Ok(user) => user,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let link_id = match User::create_confirmation(&db.0, &user, &payload.email).await {
        Ok(link_id) => link_id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    println!(
        "Sending Email with link_id {:?} to {:?}",
        link_id, payload.email
    );
    StatusCode::CREATED
}

#[derive(Deserialize)]
struct Confirm {
    link_id: String,
}
#[derive(Serialize)]
struct SessionToken {
    token: String,
}
pub struct Confirmation {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) link_id: Uuid,
    pub(crate) email: String,
    pub(crate) created: NaiveDateTime,
}
async fn signup_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let confirmation =
        match User::get_by_link_id(&db.0, Uuid::parse_str(&payload.link_id).unwrap()).await {
            Ok(conf) => conf,
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    User::delete_confirmation(&db.0, confirmation.id)
        .await
        .unwrap();
    let mut user = User::get_by_id(&db.0, confirmation.user_id).await.unwrap();
    user.email = Some(confirmation.email);
    user.update(&db.0).await.unwrap();

    let token = User::create_session(&db.0, &user).await.unwrap();
    Ok((StatusCode::CREATED, Json(SessionToken { token })))
}

//get user

#[derive(Deserialize)]
struct SignInJson {
    email: String,
    password: String,
}
async fn signin(
    db: Extension<DbPool>,
    Json(payload): Json<SignInJson>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let user = User::get_by_email(&db.0, &payload.email).await.unwrap();
    if user.password == payload.password {
        let token = User::create_session(&db.0, &user).await.unwrap();
        return Ok((StatusCode::CREATED, Json(SessionToken { token })));
    }
    Err(StatusCode::NOT_FOUND)
}
/*
async fn signout(Path(params): &str) {
    let token = params.get("token");

    todo!();
}

async fn forgot(Path(params): &str) {
    let email = params.get("email");

    todo!();
}

async fn forgot_confirm(Path(params): &str) {
    let email = params.get("email");
    let password = params.get("password");

    todo!();
}

async fn change_email(Path(params): &str) {
    let new_email = params.get("email");

    todo!();
}

async fn delete_user(Path(params): &str) {
    let password = params.get("password");

    todo!();
}
*/
