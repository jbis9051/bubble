use crate::routes;

use crate::config::CONFIG;
use crate::types::{DbPool, EmailServiceArc};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::Row;

pub fn router(pool: DbPool, email_service: EmailServiceArc) -> Router {
    let v1 = Router::new()
        .nest("/user", routes::user::router())
        .nest("/client", routes::client::router())
        .nest("/message", routes::message::router());

    Router::new()
        .route("/", get(status))
        .route("/reset", get(reset))
        .nest("/v1", v1)
        .layer(Extension(pool))
        .layer(Extension(email_service))
}

async fn status() -> (StatusCode, String) {
    (StatusCode::OK, "Ok".to_string())
}

async fn reset(db: Extension<DbPool>) -> StatusCode {
    // WARNING: This is a debug endpoint that resets the database.
    // It should not be used in production. To be honest, we should probably remove it before we go live.
    if !CONFIG.debug_mode {
        return StatusCode::NOT_FOUND;
    }
    let num_users = sqlx::query("SELECT COUNT(*) FROM user")
        .fetch_one(&db.0)
        .await
        .unwrap()
        .get::<i64, _>(0);
    if num_users > 10 {
        panic!("Too many users in the database. This is probably a mistake.");
    }

    let query: Vec<&str> = r#"
DELETE FROM "key_package";
DELETE FROM "recipient";
DELETE FROM "message";
DELETE FROM "forgot";
DELETE FROM "confirmation";
DELETE FROM "session";
DELETE FROM "user";
DELETE FROM "client";
"#
    .split('\n')
    .collect();

    for query in query {
        sqlx::query(query).execute(&db.0).await.unwrap();
    }

    println!("Database reset.");

    StatusCode::OK
}
