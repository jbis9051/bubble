use crate::routes;
use crate::types::DbPool;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::Serialize;

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(hello))
        .nest("/user", routes::user::router())
        .nest("/group", routes::group::router())
        .layer(Extension(pool))
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
