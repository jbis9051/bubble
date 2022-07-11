use crate::routes;
use crate::types::DbPool;
use axum::{Extension, Router};

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .nest("/user", routes::user::router())
        .nest("/group", routes::group::router())
        .layer(Extension(pool))
}
