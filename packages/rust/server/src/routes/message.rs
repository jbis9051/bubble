use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::message::Message;

use crate::types::{Base64, DbPool};
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
    pub message: Base64,
}

async fn send_message(
    db: Extension<DbPool>,
    Json(payload): Json<MessageRequest>,
    _: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuids = payload
        .client_uuids
        .into_iter()
        .map(|uuid| Uuid::parse_str(&uuid))
        .collect::<Result<Vec<Uuid>, uuid::Error>>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let clients = Client::filter_uuids(&db, &uuids)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    if clients.len() != uuids.len() {
        return Err(StatusCode::NOT_FOUND);
    }
    let client_ids: Vec<_> = clients.iter().map(|client| client.id).collect();

    let mut message = Message {
        id: Default::default(),
        message: payload.message.0,
        created: NaiveDateTime::from_timestamp(0, 0),
    };
    message
        .create(&db, &client_ids)
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
    pub messages: Vec<Base64>,
}

async fn receive_message(
    db: Extension<DbPool>,
    Json(payload): Json<CheckMessages>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<MessagesReturned>), StatusCode> {
    // Get client
    let uuid = Uuid::parse_str(&payload.client_uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db, &uuid)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    if client.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let messages = Message::from_client_id(&db, client.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    if messages.is_empty() {
        return Ok((
            StatusCode::OK,
            Json(MessagesReturned {
                messages: Vec::new(),
            }),
        ));
    }
    let ids: Vec<_> = messages.iter().map(|message| message.id).collect();
    Message::delete_ids(&ids, client.id, &db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let messages_to_return = messages
        .iter()
        .map(|message| Base64(message.message.clone()))
        .collect();

    Ok((
        StatusCode::OK,
        Json(MessagesReturned {
            messages: messages_to_return,
        }),
    ))
}
