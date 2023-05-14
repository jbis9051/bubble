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

pub fn router() -> Router {
    Router::new().route("/", get(receive_message).post(send_message))
}

#[derive(Serialize, Deserialize)]
pub struct MessageRequest {
    pub client_uuids: Vec<String>,
    pub message: Vec<u8>,
}

// list of recipients, message,
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

    // TODO O(n) -> O(1)
    for client_uuid in payload.client_uuids {
        let client = Client::from_uuid(
            &db.0,
            &Uuid::parse_str(&client_uuid).map_err(|_| StatusCode::BAD_REQUEST)?,
        )
        .await
        .map_err(map_sqlx_err)?;

        let mut recipient = Recipient {
            id: Default::default(),
            client_id: client.id,
            message_id: message.id,
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        recipient.create(&db.0).await.map_err(map_sqlx_err)?;
    }

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

// Receive message also deletes message from table
async fn receive_message(
    db: Extension<DbPool>,
    Json(payload): Json<CheckMessages>,
    user: AuthenticatedUser,
) -> Result<(StatusCode, Json<MessagesReturned>), StatusCode> {
    // Get client
    let uuid = &Uuid::parse_str(&payload.client_uuid).map_err(|_| StatusCode::BAD_REQUEST)?;
    let client = Client::from_uuid(&db.0, uuid).await.map_err(map_sqlx_err)?;

    // Ensure client belongs to user
    if client.user_id != user.id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Get Recipients
    let recipients = Recipient::filter_client_id(&db.0, client.id)
        .await
        .map_err(map_sqlx_err)?;

    // TODO O(n) -> O(1)
    // Get messages
    let mut messages_to_return = Vec::new();
    for recipient in recipients {
        let message_to_read = Message::from_id(&db.0, recipient.message_id)
            .await
            .map_err(map_sqlx_err)?;
        messages_to_return.push(message_to_read.message.clone());
        message_to_read.delete(&db.0).await.map_err(map_sqlx_err)?;
        recipient.delete(&db.0).await.map_err(map_sqlx_err)?;
    }

    Ok((
        StatusCode::OK,
        Json(MessagesReturned {
            messages: messages_to_return,
        }),
    ))
}
