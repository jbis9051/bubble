use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::Router;
use axum::{Extension, Json};

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::confirmation::Confirmation;

use crate::models::forgot::Forgot;
use crate::models::session::Session;
use crate::models::user::User;
use crate::types::DbPool;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/signup-confirm", post(signup_confirm))
        .route("/signin", post(signin))
        .route("/signout", delete(signout))
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
    let user = User {
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
    let user = match user.create(&db.0).await {
        Ok(user) => user,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let conf = Confirmation {
        id: 0,
        user_id: user.id,
        link_id: Uuid::new_v4(),
        email: payload.email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let conf = match conf.create(&db.0).await {
        Ok(link_id) => link_id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    println!(
        "Sending Email with link_id {:?} to {:?}",
        conf.link_id, conf.email
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

async fn signup_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let conf =
        match Confirmation::from_link_id(&db.0, Uuid::parse_str(&payload.link_id).unwrap()).await {
            Ok(conf) => conf,
            Err(e) => {
                let status = match e {
                    sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                return Err(status);
            }
        };

    match conf.delete(&db.0).await {
        Ok(_) => (),
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    }

    let mut user = match User::from_id(&db.0, conf.user_id).await {
        Ok(user) => user,
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    };

    user.email = Some(conf.email);
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

    let session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let session = match session.create(&db.0).await {
        Ok(token) => token,
        Err(e) => {
            let status = match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(status);
        }
    };

    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
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
    let user = User::from_email(&db.0, &payload.email).await.unwrap();
    if user.password == payload.password {
        let session = Session {
            id: 0,
            user_id: user.id,
            token: Uuid::new_v4(),
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        let session = session.create(&db.0).await.unwrap();

        return Ok((
            StatusCode::CREATED,
            Json(SessionToken {
                token: session.token.to_string(),
            }),
        ));
    }
    Err(StatusCode::NOT_FOUND)
}

async fn signout(db: Extension<DbPool>, Json(payload): Json<SessionToken>) -> StatusCode {
    let session = Session::from_token(&db.0, &payload.token).await.unwrap();
    session.delete(&db.0).await.unwrap();
    StatusCode::OK
}

#[derive(Deserialize)]
struct Email {
    email: String,
}

async fn forgot(db: Extension<DbPool>, Json(payload): Json<Email>) -> StatusCode {
    let user = User::from_email(&db.0, &payload.email).await.unwrap();
    let forgot = Forgot {
        id: 0,
        user_id: user.id,
        forgot_id: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    let forgot = forgot.create(&db.0).await.unwrap();

    println!(
        "Sending email with {:?} to {:?}",
        forgot.forgot_id, user.email
    );
    StatusCode::CREATED
}

#[derive(Deserialize)]
struct ForgotConfirm {
    password: String,
    forgot_code: String,
}

async fn forgot_confirm(db: Extension<DbPool>, Json(payload): Json<ForgotConfirm>) -> StatusCode {
    let forgot = Forgot::from_uuid(&db.0, &payload.forgot_code)
        .await
        .unwrap();
    let mut user = User::from_id(&db.0, forgot.user_id).await.unwrap();
    forgot.delete(&db.0).await.unwrap();
    user.password = payload.password;
    user.update(&db.0).await.unwrap();
    StatusCode::CREATED
}
/*
async fn change_email(db: Extension<DbPool>, Json(payload): Json<Email>) -> StatusCode {
    let user = User::get_by_email(&db.0, &payload.email).await.unwrap();
    let link_id = User::create_confirmation(&db.0, &user, &payload.email)
        .await
        .unwrap();

    println!("Sending {:?} to {:?}", link_id, &payload.email);
    StatusCode::CREATED
}


async fn change_email_confirm() {}
async fn delete_user(Path(params): &str) {
    let password = params.get("password");

    todo!();
}
*/
