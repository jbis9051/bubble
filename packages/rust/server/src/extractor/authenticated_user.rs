use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

use crate::models::user::User;
use crate::types::DbPool;

pub struct AuthenticatedUser(pub(crate) User);

impl Deref for AuthenticatedUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AuthenticatedUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
            User::from_session(
                &db.0,
                Uuid::parse_str(token.token())
                    .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?,
            )
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?,
        ))
    }
}
