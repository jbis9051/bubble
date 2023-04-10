use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;
use rand_core::OsRng;

pub fn hash(password: &str) -> String {
    let byte_password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(byte_password, &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string()
}

pub fn verify(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let correct = PasswordHash::new(&hash)?;
    let password = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(password, &correct)
        .is_ok())
}
