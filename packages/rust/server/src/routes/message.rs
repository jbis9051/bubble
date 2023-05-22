use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::message::Message;
use crate::models::recipient::Recipient;
use crate::routes::map_sqlx_err;
use crate::types::DbPool;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use std::iter::Iterator;

pub fn router() -> Router {
    Router::new().route("/", get(receive_message).post(send_message))
}

#[derive(Serialize, Deserialize)]
pub struct MessageRequest {
    pub client_uuids: Vec<String>,
    pub message: Vec<u8>,
}

async fn send_message(
    db: Extension<DbPool>,
    Json(payload): Json<MessageRequest>,
    _: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let mut message = Message {
        id: Default::default(),
        message: payload.message,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    message.create(&db.0).await.map_err(map_sqlx_err)?;

    let uuids: Result<Vec<Uuid>, StatusCode> = payload
        .client_uuids
        .iter()
        .map(|uuid| Uuid::parse_str(uuid).map_err(|_| StatusCode::BAD_REQUEST))
        .collect();

    let clients: Vec<Client> = Client::filter_uuids(&db.0, &uuids?)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let client_ids = clients.iter().map(|client| client.id).collect();

    Recipient::create_all(&db.0, client_ids, message.id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct CheckMessages {
    pub client_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessagesReturned {
    pub messages: Vec<Vec<u8>>,
}

async fn receive_message(
    db: Extension<DbPool>,
    Json(payload): Json<CheckMessages>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<MessagesReturned>), StatusCode> {
    // Get client
    let uuid = Uuid::parse_str(&payload.client_uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db.0, &uuid)
        .await
        .map_err(map_sqlx_err)?;

    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }
    let messages_to_read = Message::from_client_id(&db.0, client.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;
    let recipients = Recipient::filter_client_id(&db.0, client.id)
        .await
        .map_err(map_sqlx_err)?;
    if recipients.is_empty() || messages_to_read.is_empty() {
        return Ok((
            StatusCode::OK,
            Json(MessagesReturned {
                messages: Vec::new(),
            }),
        ));
    }

    let messages_to_return = messages_to_read
        .iter()
        .map(|message| message.message.clone())
        .collect();

    Recipient::delete_ids(
        recipients.iter().map(|recipient| recipient.id).collect(),
        &db.0,
    )
    .await
    .map_err(map_sqlx_err)?;
    Message::delete_ids(
        &messages_to_read.iter().map(|message| message.id).collect(),
        &db.0,
    )
    .await
    .map_err(map_sqlx_err)?;

    Ok((
        StatusCode::OK,
        Json(MessagesReturned {
            messages: messages_to_return,
        }),
    ))
}
