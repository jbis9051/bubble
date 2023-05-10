use axum::body::HttpBody;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use axum::{Extension, Json};
use ed25519_dalek::{Digest, PublicKey, Signature, Verifier};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::key_package::KeyPackage as KeyPackageModel;
use crate::routes::map_sqlx_err;
use crate::services::user::get_user_identity;
use crate::types::DbPool;
use openmls::key_packages::KeyPackage;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:uuid", get(get_client).put(update))
}

#[derive(Serialize, Deserialize)]
pub struct PublicClient {
    pub signing_key: Vec<u8>,
    pub signature: Vec<u8>,
}

pub async fn create(
    db: Extension<DbPool>,
    Json(payload): Json<PublicClient>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, String), StatusCode> {
    let identity = get_user_identity(&user)
        .ok_or(StatusCode::FAILED_DEPENDENCY)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // ensure the user has a valid identity key

    let signature =
        Signature::from_bytes(&payload.signature).map_err(|_| StatusCode::BAD_REQUEST)?;

    identity
        .verify(&payload.signing_key, &signature)
        .map_err(|_| StatusCode::BAD_REQUEST)?; // ensure the signature is valid

    let mut client = Client {
        id: 0,
        user_id: user.id,
        uuid: Uuid::new_v4(),
        signing_key: payload.signing_key,
        signature: payload.signature,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    client.create(&db).await.map_err(map_sqlx_err)?;

    Ok((StatusCode::CREATED, client.uuid.to_string()))
}

pub async fn update(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<PublicClient>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }
    let identity = get_user_identity(&user)
        .ok_or(StatusCode::FAILED_DEPENDENCY)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let signature =
        Signature::from_bytes(&payload.signature).map_err(|_| StatusCode::BAD_REQUEST)?;

    identity
        .verify(&payload.signing_key, &signature)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    client.signing_key = payload.signing_key;
    client.signature = payload.signature;

    client.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

pub async fn get_client(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    _: AuthenticatedUser,
) -> Result<Json<PublicClient>, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    Ok(Json(PublicClient {
        signing_key: client.signing_key,
        signature: client.signature,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ReplaceKeyPackages {
    pub key_packages: Vec<Vec<u8>>,
}

pub async fn replace_key_packages(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<ReplaceKeyPackages>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    for package in &payload.key_packages {
        let key_package =
            KeyPackage::try_from(package.as_slice()).map_err(|_| StatusCode::BAD_REQUEST)?;
        if key_package.credential().identity()
            != format!("keypackage_{}_{}", client.uuid, user.uuid).as_bytes()
        {
            // IMPORTANT: we validate that the key package is actually for this client. This identifier will be used on the client.
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    KeyPackageModel::delete_all_by_client_id(&db, client.id)
        .await
        .map_err(map_sqlx_err)?;

    for package in payload.key_packages {
        let mut key_package = KeyPackageModel {
            id: 0,
            client_id: client.id,
            key_package: package,
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        key_package.create(&db).await.map_err(map_sqlx_err)?;
    }

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct KeyPackagePublic {
    pub key_package: Vec<u8>,
}

pub async fn get_key_package(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    _: AuthenticatedUser,
) -> Result<Json<KeyPackagePublic>, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    let key_package = KeyPackageModel::get_one(&db, client.id)
        .await
        .map_err(map_sqlx_err)?
        .ok_or(StatusCode::NOT_FOUND)?; // TODO better error

    // TODO handle case where key package is last one

    key_package.delete(&db).await.map_err(map_sqlx_err)?; // these are one time use to we delete them

    Ok(Json(KeyPackagePublic {
        key_package: key_package.key_package,
    }))
}
