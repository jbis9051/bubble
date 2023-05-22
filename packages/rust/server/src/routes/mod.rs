use axum::http::StatusCode;

pub mod client;
pub mod message;

pub mod client;
pub mod user;

pub fn map_sqlx_err(err: sqlx::Error) -> StatusCode {
    match err {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
