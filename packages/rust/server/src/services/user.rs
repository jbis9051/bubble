use crate::models::user::User;
use ed25519_dalek::PublicKey;

pub fn get_user_identity(user: &User) -> Option<Result<PublicKey, ed25519_dalek::SignatureError>> {
    user.identity
        .as_ref()
        .map(|identity| PublicKey::from_bytes(identity.as_slice()))
}
