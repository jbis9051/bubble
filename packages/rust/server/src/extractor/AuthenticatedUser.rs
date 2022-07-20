use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::models::user::User;
use crate::types::DbPool;

pub struct AuthenticatedUser(pub(crate) User);

#[async_trait]
impl<B> FromRequest<B> for AuthenticatedUser
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let db = Extension::<DbPool>::from_request(req)
            .await
            .map_err(|err| err.into_response())?;

        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| err.into_response())?;

        Ok(AuthenticatedUser(
            User::user_from_session(&db.0, token.token())
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?,
        ))
    }
}
