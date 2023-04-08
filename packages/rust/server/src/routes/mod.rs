use crate::routes::user::BadGroup;
use axum::http::StatusCode;
use axum::Json;
pub mod group;
pub mod user;

pub fn map_sqlx_err(err: sqlx::Error) -> StatusCode {
    match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn map_bad_group_err(err: sqlx::Error) -> (StatusCode, Json<BadGroup>) {
    match err {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(BadGroup {
                group: "".to_string(),
            }),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BadGroup {
                group: "".to_string(),
            }),
        ),
    }
}
