use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use axum::{Extension, Json};

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::user::User;
use crate::types::DbPool;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/signup-confirm", post(signup_confirm))
        .route("/signin", post(signin))
        .route("/signout", post(signout))
        .route("/forgot", post(forgot))
        .route("/forgot-confirm", post(forgot_confirm))
    /*.route("/change-email", post(change_email))
    .route("/delete", delete(delete_user))*/
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub phone: Option<String>,
    pub name: String,
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
    //TODO add verification that email being used to create "confirmation" table is not in "user" table
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

#[derive(Serialize, Deserialize)]
pub struct Confirm {
    pub link_id: String,
}
#[derive(Serialize, Deserialize)]
struct SessionToken {
    pub token: String,
}
pub struct Confirmation {
    pub id: i32,
    pub user_id: i32,
    pub link_id: Uuid,
    pub email: String,
    pub created: NaiveDateTime,
}

async fn signup_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let confirmation = match User::get_by_link_id(
        &db.0,
        Uuid::parse_str(&payload.link_id).expect("Couldn't read link_id to UUID"),
    )
    .await
    {
        Ok(conf) => conf,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match User::delete_confirmation(&db.0, confirmation.id).await {
        Ok(_) => (),
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    }

    let mut user = match User::get_by_id(&db.0, confirmation.user_id).await {
        Ok(user) => user,
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    };

    user.email = Some(confirmation.email);
    match user.update(&db.0).await {
        Ok(_) => (),
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    }

    let token = match User::create_session(&db.0, &user).await {
        Ok(token) => token,
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    };

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

async fn signout(db: Extension<DbPool>, Json(payload): Json<SessionToken>) -> StatusCode {
    User::delete_session(&db.0, &payload.token).await.unwrap();
    StatusCode::OK
}

#[derive(Deserialize)]
struct Email {
    email: String,
}
async fn forgot(db: Extension<DbPool>, Json(payload): Json<Email>) -> StatusCode {
    let user = User::get_by_email(&db.0, &payload.email).await.unwrap();
    let forgot = User::create_forgot(&db.0, &user).await.unwrap();

    println!("Sending email with {:?} to {:?}", forgot, user.email);
    StatusCode::CREATED
}

#[derive(Deserialize)]
struct ForgotConfirm {
    email: String,
    password: String,
    forgot_id: String,
}
pub struct ForgotRow {
    pub id: i32,
    pub user_id: i32,
    pub forgot_id: Uuid,
    pub created: NaiveDateTime,
}
async fn forgot_confirm(db: Extension<DbPool>, Json(payload): Json<ForgotConfirm>) -> StatusCode {
    let row = User::get_forgot(&db.0, &payload.forgot_id).await.unwrap();
    let mut user = User::get_by_email(&db.0, &payload.email).await.unwrap();
    User::delete_forgot(&db.0, &payload.forgot_id)
        .await
        .unwrap();
    user.password = payload.password;
    user.update(&db.0).await.unwrap();
    StatusCode::CREATED
}

async fn change_email(db: Extension<DbPool>, Json(payload): Json<Email>) -> StatusCode {
    let user = User::get_by_email(&db.0, &payload.email).await.unwrap();
    let link_id = User::create_confirmation(&db.0, &user, &payload.email)
        .await
        .unwrap();

    println!("Sending {:?} to {:?}", link_id, &payload.email);
    StatusCode::CREATED
}
/*

async fn change_email_confirm() {}
async fn delete_user(Path(params): &str) {
    let password = params.get("password");

    todo!();
}
*/
