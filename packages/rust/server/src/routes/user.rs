use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::Router;
use axum::{Extension, Json};

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::body::HttpBody;
use rand_core::OsRng;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::confirmation::Confirmation;
use crate::models::member::Member;

use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::forgot::Forgot;
use crate::models::group::Group;
use crate::models::session::Session;
use crate::models::user::User;
use crate::routes::{map_bad_group_err, map_sqlx_err};
use crate::types::DbPool;
use serde::{Deserialize, Serialize};
use sqlx::Row;

pub fn router() -> Router {
    Router::new()
        .route("/", delete(delete_user))
        .route("/signup", post(signup))
        .route("/signup-confirm", post(signup_confirm))
        .route("/signin", post(signin))
        .route("/signout", delete(signout))
        .route("/forgot", post(forgot))
        .route("/forgot-confirm", post(forgot_confirm))
        .route("/email", post(change_email))
        .route("/email-confirm", post(change_email_confirm))
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
    let byte_password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(byte_password, &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let mut user = User {
        id: 0,
        uuid: Uuid::new_v4(),
        username: payload.username,
        password: Some(password),
        profile_picture: None,
        email: None,
        phone: payload.phone,
        name: payload.name,
        created: NaiveDateTime::from_timestamp(0, 0),
        deleted: None,
    };
    user.create(&db.0).await.map_err(map_sqlx_err)?;

    let mut conf = Confirmation {
        id: 0,
        user_id: user.id,
        link_id: Uuid::new_v4(),
        email: payload.email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    conf.create(&db.0).await.map_err(map_sqlx_err)?;

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
        &Uuid::parse_str(&payload.link_id).map_err(|_| StatusCode::BAD_REQUEST)?,
    )
    .await
    .map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db.0, conf.user_id)
        .await
        .map_err(map_sqlx_err)?;

    Confirmation::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;
    user.email = Some(conf.email);
    user.update(&db.0).await.map_err(map_sqlx_err)?;

    let mut session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    session.create(&db.0).await.map_err(map_sqlx_err)?;

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
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !user.verify_password(&payload.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    session.create(&db.0).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
}

async fn signout(
    db: Extension<DbPool>,
    Json(payload): Json<SessionToken>,
    _user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
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
    Json(payload): Json<Email>,
) -> Result<StatusCode, StatusCode> {
    let user = User::from_email(&db.0, &payload.email)
        .await
        .map_err(map_sqlx_err)?;
    let mut forgot = Forgot {
        id: 0,
        user_id: user.id,
        forgot_id: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    forgot.create(&db.0).await.map_err(map_sqlx_err)?;

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
    let forgot = Forgot::from_forgot_id(&db.0, &uuid)
        .await
        .map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db.0, forgot.user_id)
        .await
        .map_err(map_sqlx_err)?;

    Forgot::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;

    Session::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;

    let byte_password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(byte_password, &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    user.password = Some(password);
    user.update_password(&db.0).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct ChangeEmail {
    pub new_email: String,
    pub password: String,
}

async fn change_email(
    db: Extension<DbPool>,
    Json(payload): Json<ChangeEmail>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if !user.0.verify_password(&payload.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut change = Confirmation {
        id: 0,
        user_id: user.0.id,
        link_id: Uuid::new_v4(),
        email: payload.new_email,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    change.create(&db.0).await.map_err(map_sqlx_err)?;

    println!("Sending code {:?} to {:?}", change.link_id, change.email);

    Ok(StatusCode::CREATED)
}

async fn change_email_confirm(
    db: Extension<DbPool>,
    Json(payload): Json<Confirm>,
) -> Result<(StatusCode, Json<SessionToken>), StatusCode> {
    let confirmation = Confirmation::from_link_id(
        &db.0,
        &Uuid::parse_str(&payload.link_id).map_err(|_| StatusCode::BAD_REQUEST)?,
    )
    .await
    .map_err(map_sqlx_err)?;
    let mut user = User::from_id(&db.0, confirmation.user_id)
        .await
        .map_err(map_sqlx_err)?;

    Confirmation::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;
    Session::delete_all(&db.0, user.id)
        .await
        .map_err(map_sqlx_err)?;
    user.email = Some(confirmation.email.clone());
    user.update(&db.0).await.map_err(map_sqlx_err)?;

    let mut session = Session {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    session.create(&db.0).await.map_err(map_sqlx_err)?;
    Ok((
        StatusCode::CREATED,
        Json(SessionToken {
            token: session.token.to_string(),
        }),
    ))
}

#[derive(Serialize, Deserialize)]
pub struct DeleteJson {
    pub password: String,
}

//If "group" is blank, then only consider the StatusCode error
#[derive(Serialize, Deserialize)]
pub struct BadGroup {
    pub group: String,
}

async fn delete_user(
    db: Extension<DbPool>,
    Json(payload): Json<DeleteJson>,
    mut user: AuthenticatedUser,
) -> Result<StatusCode, (StatusCode, Json<BadGroup>)> {
    if !user.0.verify_password(&payload.password) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(BadGroup {
                group: "".to_string(),
            }),
        ));
    }

    //Checking if only one admin is in the group. If so, we return a B
    let groups = Member::group_id(&db, user.0.id)
        .await
        .map_err(map_bad_group_err)?;
    let members_size_len = groups.len();
    for i in groups {
        let group_id: i32 = i.get("group_id");
        let admin_size = Member::admin_num(&db, group_id)
            .await
            .map_err(map_bad_group_err)?;
        if admin_size.len() == 1 {
            let admin_id: i32 = admin_size[0].get("user_id");
            if admin_id == user.0.id && members_size_len != 1 {
                let bad_group = Group::from_id(&db, group_id)
                    .await
                    .map_err(map_bad_group_err)?;
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(BadGroup {
                        group: bad_group.group_name,
                    }),
                ));
            }
        }
        //Checking if admin is last member in the group, if so the group will be deleted
        let members = Member::all_members_in_group(&db, group_id)
            .await
            .map_err(map_bad_group_err)?;
        if members.len() == 1 {
            let group = Group::from_id(&db, group_id)
                .await
                .map_err(map_bad_group_err)?;
            group.delete(&db).await.map_err(map_bad_group_err)?;
        }
    }

    user.0.delete(&db.0).await.map_err(map_bad_group_err)?;
    Ok(StatusCode::OK)
}
