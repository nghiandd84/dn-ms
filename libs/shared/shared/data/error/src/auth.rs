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

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum AuthError {
    #[error("Context not found in request extensions")]
    NotFoundUser,
    #[error("Login failed due to invalid credentials")]
    WrongPassword,
    #[error("Logout failed")]
    Unknow,
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
