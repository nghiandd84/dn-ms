use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Error)]
pub enum PasswordError {
    #[error("The provided password is empty")]
    EmptyPassword,
    #[error("Error occurred during password hashing")]
    HashingError,
    #[error("The provided hash format is invalid")]
    InvalidHashFormat,
}
