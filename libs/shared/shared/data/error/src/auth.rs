use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("The provided token is invalid")]
    InvalidToken,
    #[error("The token has expired")]
    ExpiredToken,
    #[error("Unauthorized access attempt")]
    UnauthorizedAccess,
    #[error("Can not create token")]
    CanNotCreateToken,
}

#[derive(Debug, Serialize, Deserialize, Error, Clone)]
#[serde(tag = "message", content = "details", rename_all = "snake_case")]
pub enum AuthError {
    #[error("Not found user")]
    NotFoundUser,
    #[error("Wrong password")]
    WrongPassword,
    #[error("User already exists")]
    ExistingUser,
    #[error("Unknow role")]
    UnknowRole,
    #[error("Insufficient permission")]
    InsufficientPermission,
    #[error("Unknow error")]
    Unknow,
}

impl AuthError {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            AuthError::NotFoundUser => StatusCode::NOT_FOUND,
            AuthError::UnknowRole => StatusCode::NOT_FOUND,
            AuthError::WrongPassword => StatusCode::UNAUTHORIZED,
            AuthError::InsufficientPermission => StatusCode::FORBIDDEN,
            AuthError::ExistingUser => StatusCode::CONFLICT,
            AuthError::Unknow => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// Define a custom error type for our extractor
#[derive(Debug)]
pub enum HeaderError {
    MissingHeader,
    EmptyHeader,
    Unauthorized,        // Added for cache check failure
    InternalServerError, // For unexpected errors during cache lookup
}

// Implement IntoResponse for our custom error, so it can be returned directly from the extractor.
// This will map our custom errors to HTTP responses.
impl IntoResponse for HeaderError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HeaderError::MissingHeader => (StatusCode::UNAUTHORIZED, "Missing header"),
            HeaderError::EmptyHeader => (StatusCode::UNAUTHORIZED, "Empty header"),
            HeaderError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "User ID not authorized or found in cache",
            ),
            HeaderError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error during user ID validation",
            ),
        };

        // Return a JSON error response
        let body = Json(serde_json::json!({
            "error": error_message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
