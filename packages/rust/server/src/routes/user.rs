use axum::body::HttpBody;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use axum::{Extension, Json};
use ed25519_dalek::PublicKey;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::models::confirmation::Confirmation;

use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::forgot::Forgot;
use crate::models::session::Session;
use crate::models::user::User;
use crate::routes::map_sqlx_err;
use crate::services::email::Recipient;

use crate::config::CONFIG;
use crate::services::password;
use crate::services::session::create_session;
use crate::types::{DbPool, EmailServiceArc};
use common::base64::Base64;
use common::http_types::{
    ChangeEmail, ClientsResponse, ConfirmEmail, CreateUser, CreateUserResponse, DeleteUser,
    ForgotEmail, Login, PasswordReset, PasswordResetCheck, PublicClient, PublicUser, Search,
    SearchResponse, SessionTokenRequest, SessionTokenResponse, UpdateIdentity, UserProfile,
};

pub fn router() -> Router {
    Router::new()
        .route("/", delete(delete_user))
        .route("/register", post(register))
        .route("/confirm", patch(confirm))
        .route("/session", post(login).delete(logout))
        .route("/forgot", post(forgot))
        .route("/reset", get(reset_check).patch(reset))
        .route("/email", post(change_email))
        .route("/identity", put(update_identity))
        .route("/:uuid", get(get_user))
        .route("/:uuid/clients", get(get_clients))
        .route("/profile", put(update_profile))
        .route("/search", get(search))
}

async fn register(
    db: Extension<DbPool>,
    email_service: Extension<EmailServiceArc>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<CreateUserResponse>), (StatusCode, String)> {
    // so technically there is race condition here, but I'm too lazy to avoid it

    if (User::from_username(&db, &payload.username).await).is_ok() {
        return Err((StatusCode::CONFLICT, "username".to_string()));
    }
    if (User::from_email(&db, &payload.email).await).is_ok() {
        return Err((StatusCode::CONFLICT, "email".to_string()));
    }

    let hash = password::hash(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to hash password".to_string(),
        )
    })?;

    PublicKey::from_bytes(&payload.identity).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "unable to parse identity".to_string(),
        )
    })?;

    let mut user = User {
        id: 0,
        uuid: Uuid::new_v4(),
        username: payload.username,
        password: hash,
        email: None,
        name: payload.name,
        identity: payload.identity.0,
        primary_client_id: None,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    user.create(&db).await.map_err(|_| {
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

    confirmation.create(&db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to create confirmation".to_string(),
        )
    })?;

    email_service
        .send(
            "Bubble - Email Confirmation",
            &[Recipient {
                address: confirmation.email,
                name: user.name,
            }],
            Some(&format!(
                "Please click the link to confirm your email address: {}",
                confirmation.token
            )),
            None,
        ) // successful confirmation email
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to send email".to_string(),
            )
        })?;

    if CONFIG.debug_mode {
        // WARNING: this is a debug mode only feature, do not use in production
        confirm(
            db,
            Json(ConfirmEmail {
                token: confirmation.token,
            }),
        )
        .await
        .map_err(|e| {
            println!("unable to confirm email: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to confirm email".to_string(),
            )
        })?;
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateUserResponse {
            user_uuid: user.uuid,
        }),
    ))
}

async fn confirm(
    db: Extension<DbPool>,
    Json(payload): Json<ConfirmEmail>,
) -> Result<(StatusCode, Json<SessionTokenResponse>), StatusCode> {
    let confirmation = Confirmation::from_token(&db, &payload.token)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    confirmation.delete(&db).await.map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db, confirmation.user_id)
        .await
        .map_err(map_sqlx_err)?;
    user.email = Some(confirmation.email);
    user.update(&db).await.map_err(map_sqlx_err)?;

    // whenever we change a user's email we should invalidate all tokens

    Session::delete_all(&db, user.id)
        .await
        .map_err(map_sqlx_err)?;

    let token = create_session(&db, user.id).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::OK,
        Json(SessionTokenResponse {
            user_uuid: user.uuid,
            bearer: token,
        }),
    ))
}

async fn login(
    db: Extension<DbPool>,
    Json(payload): Json<Login>,
) -> Result<(StatusCode, Json<SessionTokenResponse>), StatusCode> {
    let user = {
        let by_email = User::from_email(&db, &payload.username_or_email).await;
        if let Ok(user) = by_email {
            user
        } else {
            User::from_username(&db, &payload.username_or_email)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
        }
    };

    if !password::verify(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_session(&db, user.id).await.map_err(map_sqlx_err)?;

    Ok((
        StatusCode::CREATED,
        Json(SessionTokenResponse {
            user_uuid: user.uuid,
            bearer: token,
        }),
    ))
}

async fn logout(
    db: Extension<DbPool>,
    Json(payload): Json<SessionTokenRequest>,
    _user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    // so while with authenticate the user, this permits any user to delete any session token
    // formally, this could be considered a problem (users should only be able to delete *their* session tokens),
    // but in reality if you have another users session token, you could repeat this request with that token
    // and delete their session token anyway, so it's not really a problem

    let session = Session::from_token(&db, &payload.token)
        .await
        .map_err(map_sqlx_err)?;
    session.delete(&db).await.map_err(map_sqlx_err)?;
    Ok(StatusCode::OK)
}

async fn forgot(
    db: Extension<DbPool>,
    email_service: Extension<EmailServiceArc>,
    Json(payload): Json<ForgotEmail>,
) -> Result<StatusCode, StatusCode> {
    let user = User::from_email(&db, &payload.email)
        .await
        .map_err(|_| StatusCode::CREATED)?;

    let mut forgot = Forgot {
        id: 0,
        user_id: user.id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    forgot.create(&db).await.map_err(map_sqlx_err)?;

    email_service
        .send(
            "Bubble - Password Reset",
            &[Recipient {
                address: payload.email,
                name: user.name,
            }],
            Some(&format!(
                "Please click the link to reset your password: {}",
                forgot.token
            )),
            None,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

async fn reset(
    db: Extension<DbPool>,
    Json(payload): Json<PasswordReset>,
) -> Result<StatusCode, StatusCode> {
    let uuid = payload.token;
    let forgot = Forgot::from_token(&db, &uuid).await.map_err(map_sqlx_err)?;

    let mut user = User::from_id(&db, forgot.user_id)
        .await
        .map_err(map_sqlx_err)?;

    // invalidate all forgot's and session tokens when a user successfully resets their password

    Forgot::delete_all(&db, user.id)
        .await
        .map_err(map_sqlx_err)?; // TODO: consider account hijacking attacks

    Session::delete_all(&db, user.id)
        .await
        .map_err(map_sqlx_err)?;

    user.password =
        password::hash(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

async fn reset_check(
    db: Extension<DbPool>,
    Query(payload): Query<PasswordResetCheck>,
) -> Result<StatusCode, StatusCode> {
    let uuid = payload.token;
    if Forgot::from_token(&db, &uuid).await.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

async fn change_email(
    db: Extension<DbPool>,
    email_service: Extension<EmailServiceArc>,
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

    change.create(&db).await.map_err(map_sqlx_err)?;

    email_service
        .send(
            "Bubble - Confirm Email Change",
            &[Recipient {
                address: change.email,
                name: user.0.name,
            }],
            Some(&format!(
                "Please click the link to change your email: {}",
                change.token
            )),
            None,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(
    db: Extension<DbPool>,
    Json(payload): Json<DeleteUser>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if !password::verify(&user.password, &payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    user.delete(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

async fn update_identity(
    db: Extension<DbPool>,
    Json(payload): Json<UpdateIdentity>,
    mut user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if PublicKey::from_bytes(&payload.identity).is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    user.identity = payload.identity.0;
    user.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

async fn get_user(
    db: Extension<DbPool>,
    Path(uuid): Path<Uuid>,
    _: AuthenticatedUser,
) -> Result<Json<PublicUser>, StatusCode> {
    let user = User::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    let primary_client_uuid = user
        .client(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|c| c.uuid);

    Ok(Json(PublicUser {
        uuid: user.uuid,
        username: user.username,
        name: user.name,
        primary_client_uuid,
        identity: Base64(user.identity),
    }))
}

async fn get_clients(
    db: Extension<DbPool>,
    Path(uuid): Path<Uuid>,
    _: AuthenticatedUser,
) -> Result<Json<ClientsResponse>, StatusCode> {
    let user = User::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;

    Client::filter_user_id(&db, user.id)
        .await
        .map_err(map_sqlx_err)
        .map(|clients| {
            Json(ClientsResponse {
                clients: clients
                    .into_iter()
                    .map(|c| PublicClient {
                        user_uuid: user.uuid,
                        uuid: c.uuid,
                        signing_key: Base64(c.signing_key),
                        signature: Base64(c.signature),
                    })
                    .collect(),
            })
        })
}

async fn update_profile(
    db: Extension<DbPool>,
    Json(payload): Json<UserProfile>,
    mut user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    user.name = payload.name;

    if let Some(client_uuid) = payload.primary_client_uuid {
        let client = Client::from_uuid(&db, &client_uuid)
            .await
            .map_err(map_sqlx_err)?;
        if client.user_id != user.id {
            return Err(StatusCode::UNAUTHORIZED);
        }
        user.primary_client_id = Some(client.id);
    }

    user.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

async fn search(
    db: Extension<DbPool>,
    Json(payload): Json<Search>,
    _: AuthenticatedUser,
) -> Result<Json<SearchResponse>, StatusCode> {
    let user_email = User::try_from_email(&db, &payload.query)
        .await
        .map_err(map_sqlx_err)?;

    let mut users_username = User::search_username(&db, &payload.query)
        .await
        .map_err(map_sqlx_err)?;

    let mut users_name = User::search_name(&db, &payload.query)
        .await
        .map_err(map_sqlx_err)?;

    users_username.append(&mut users_name);

    let mut users = users_username;

    users.sort_by(|a, b| a.username.cmp(&b.username));
    users.dedup_by_key(|u| u.id);

    if let Some(u) = user_email {
        users.insert(0, u);
    }

    let mut out = Vec::with_capacity(users.len());

    for user in users {
        let primary_client_uuid = user
            .client(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map(|c| c.uuid);

        out.push(PublicUser {
            uuid: user.uuid,
            username: user.username,
            name: user.name,
            primary_client_uuid,
            identity: Base64(user.identity),
        })
    }

    Ok(Json(SearchResponse { users: out }))
}
