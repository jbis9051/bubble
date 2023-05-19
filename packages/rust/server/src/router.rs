use crate::routes;
use crate::services::email::SendGridEmailService;
use crate::types::DbPool;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::Serialize;

pub fn router(pool: DbPool, email_service: SendGridEmailService) -> Router {
    Router::new()
        .route("/", get(hello))
        .nest("/user", routes::user::router())
        .nest("/client", routes::client::router())
        .layer(Extension(pool))
        .layer(Extension(email_service))
}

#[derive(Serialize)]
struct HelloInfo {
    status: String,
}

async fn hello() -> Json<HelloInfo> {
    (HelloInfo {
        status: "Hello World!".to_string(),
    })
    .into()
}
