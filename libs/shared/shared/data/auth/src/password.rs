use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum PasswordError {
    EmptyPassword,
    HashingError,
    InvalidHashFormat,
}

pub fn hash(password: impl Into<String>) -> Result<String, PasswordError> {
    let password = password.into();

    if password.is_empty() {
        return Err(PasswordError::EmptyPassword);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordError::HashingError)?
        .to_string();

    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, PasswordError> {
    if password.is_empty() {
        return Err(PasswordError::EmptyPassword);
    }

    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| PasswordError::InvalidHashFormat)?;

    let password_matches = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(password_matches)
}
