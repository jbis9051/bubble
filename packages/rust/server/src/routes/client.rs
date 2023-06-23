use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use axum::{Extension, Json};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;

use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::key_package::KeyPackage as KeyPackageModel;
use crate::models::user::User;
use crate::routes::map_sqlx_err;
use crate::types::DbPool;
use common::base64::Base64;
use common::http_types::{
    CreateClient, KeyPackagePublic, PublicClient, ReplaceKeyPackages, UpdateClient,
};
use openmls::key_packages::KeyPackage;

pub fn router() -> Router {
    Router::new()
        .route("/", post(create))
        .route(
            "/:uuid",
            get(get_client).patch(update).delete(delete_client),
        )
        .route("/:uuid/key_packages", post(replace_key_packages))
        .route("/:uuid/key_package", get(get_key_package))
}

pub async fn create(
    db: Extension<DbPool>,
    Json(payload): Json<CreateClient>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, String), StatusCode> {
    let identity =
        PublicKey::from_bytes(&user.identity).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // ensure the user has a valid identity key

    let signature =
        Signature::from_bytes(&payload.signature).map_err(|_| StatusCode::BAD_REQUEST)?;

    identity
        .verify(&payload.signing_key, &signature)
        .map_err(|_| StatusCode::BAD_REQUEST)?; // ensure the signature is valid

    let mut client = Client {
        id: 0,
        user_id: user.id,
        uuid: Uuid::new_v4(),
        signing_key: payload.signing_key.0,
        signature: payload.signature.0,
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    client.create(&db).await.map_err(map_sqlx_err)?;

    Ok((StatusCode::CREATED, client.uuid.to_string()))
}

pub async fn update(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    Json(payload): Json<UpdateClient>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let mut client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }
    let identity =
        PublicKey::from_bytes(&user.identity).map_err(|_| StatusCode::FAILED_DEPENDENCY)?;
    let signature =
        Signature::from_bytes(&payload.signature).map_err(|_| StatusCode::BAD_REQUEST)?;

    identity
        .verify(&payload.signing_key, &signature)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    client.signing_key = payload.signing_key.0;
    client.signature = payload.signature.0;

    client.update(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

pub async fn get_client(
    db: Extension<DbPool>,
    Path(uuid): Path<Uuid>,
    _: AuthenticatedUser,
) -> Result<Json<PublicClient>, StatusCode> {
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    let user = User::from_id(&db, client.user_id)
        .await
        .map_err(map_sqlx_err)?;

    Ok(Json(PublicClient {
        user_uuid: user.uuid,
        uuid: client.uuid,
        signing_key: Base64(client.signing_key),
        signature: Base64(client.signature),
    }))
}

pub async fn delete_client(
    db: Extension<DbPool>,
    Path(uuid): Path<String>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;

    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    client.delete(&db).await.map_err(map_sqlx_err)?;

    Ok(StatusCode::OK)
}

pub async fn replace_key_packages(
    db: Extension<DbPool>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<ReplaceKeyPackages>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    for package in &payload.key_packages {
        let key_package =
            KeyPackage::try_from(package.as_slice()).map_err(|_| StatusCode::BAD_REQUEST)?;
        if key_package.credential().identity()
            != format!("keypackage_{}_{}", user.uuid, client.uuid).as_bytes()
        {
            // IMPORTANT: we validate that the key package is actually for this client. This identifier will be used by other Clients to contact the Authentication Service (us)..
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
            key_package: package.0,
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        key_package.create(&db).await.map_err(map_sqlx_err)?;
    }

    Ok(StatusCode::OK)
}

pub async fn get_key_package(
    db: Extension<DbPool>,
    Path(uuid): Path<Uuid>,
    _: AuthenticatedUser,
) -> Result<Json<KeyPackagePublic>, StatusCode> {
    let client = Client::from_uuid(&db, &uuid).await.map_err(map_sqlx_err)?;
    let (key_package, count) = KeyPackageModel::get_one_with_count(&db, client.id)
        .await
        .map_err(map_sqlx_err)?;

    if count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let key_package = key_package.unwrap(); // we know there is one because of the count

    // TODO do we need to do anything if there is only one besides NOT delete it?

    if count > 1 {
        key_package.delete(&db).await.map_err(map_sqlx_err)?; // these are one time use to we delete them
    }

    Ok(Json(KeyPackagePublic {
        key_package: Base64(key_package.key_package),
    }))
}
