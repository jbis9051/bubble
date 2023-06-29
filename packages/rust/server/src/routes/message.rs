use crate::extractor::authenticated_user::AuthenticatedUser;
use crate::models::client::Client;
use crate::models::message::Message;

use crate::types::DbPool;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use axum::{Extension, Json};
use common::base64::Base64;
use common::http_types::{CheckMessages, Message as JsonMessage, MessagesResponse, SendMessage};
use sqlx::types::chrono::NaiveDateTime;
use std::iter::Iterator;

pub fn router() -> Router {
    Router::new().route("/", get(receive_message).post(send_message))
}

async fn send_message(
    db: Extension<DbPool>,
    Json(payload): Json<SendMessage>,
    _: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    let uuids = payload.client_uuids;
    if uuids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let clients = Client::filter_uuids(&db, &uuids)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    if clients.len() != uuids.len() {
        return Err(StatusCode::NOT_FOUND);
    }
    let client_ids: Vec<_> = clients.iter().map(|client| client.id).collect();

    let mut message = Message {
        id: Default::default(),
        message: payload.message.message.0,
        group_id: payload.message.group_id,
        created: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), // unwrap is safe because timestamp is 0
    };
    message
        .create(&db, &client_ids)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::OK)
}

async fn receive_message(
    db: Extension<DbPool>,
    Json(payload): Json<CheckMessages>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<MessagesResponse>), StatusCode> {
    // Get client
    let uuid = payload.client_uuid;
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
            Json(MessagesResponse {
                messages: Vec::new(),
            }),
        ));
    }
    let ids: Vec<_> = messages.iter().map(|message| message.id).collect();
    Message::delete_ids(&ids, client.id, &db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let messages_to_return = messages
        .into_iter()
        .map(|message| JsonMessage {
            message: Base64(message.message),
            group_id: message.group_id,
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(MessagesResponse {
            messages: messages_to_return,
        }),
    ))
}
