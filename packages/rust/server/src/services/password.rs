use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use rand_core::OsRng;

pub fn hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let byte_password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(byte_password, &salt)?.to_string())
}

pub fn verify(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let correct = PasswordHash::new(hash)?;
    let password = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(password, &correct)
        .is_ok())
}
