use crate::routes::user::BadGroup;
use axum::http::StatusCode;
use axum::Json;
pub mod user;

pub fn map_sqlx_err(err: sqlx::Error) -> StatusCode {
    match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
