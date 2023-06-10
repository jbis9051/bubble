use crate::routes;

use crate::types::{DbPool, EmailServiceArc};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};

pub fn router(pool: DbPool, email_service: EmailServiceArc) -> Router {
    let v1 = Router::new()
        .nest("/user", routes::user::router())
        .nest("/client", routes::client::router())
        .nest("/message", routes::message::router());

    Router::new()
        .route("/", get(status))
        .nest("/v1", v1)
        .layer(Extension(pool))
        .layer(Extension(email_service))
}

async fn status() -> (StatusCode, String) {
    (StatusCode::OK, "Ok".to_string())
}
