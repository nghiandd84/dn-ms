use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

use crate::{
    auth::{AuthError, TokenError},
    password::PasswordError,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(AuthError),
    #[error("Password error: {0}")]
    Password(PasswordError),
    #[error("Token error: {0}")]
    Token(TokenError),
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Entity not found: {entity}")]
    EntityNotFound { entity: String },
    #[error("JSON rejection error")]
    JsonRejection,
    #[error("Unknown error")]
    Unknown,
    #[error("Validation error: {0}")]
    Validation(validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

impl AppError {
    pub fn status_and_error(&self) -> (StatusCode, ClientError) {
        use self::AppError::*;
        match self {
            // Auth(ref auth_error) => match auth_error {
            //     AuthError::CtxNotInRequestExt => (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         ClientError::AuthError(AuthError::CtxNotInRequestExt),
            //     ),
            //     AuthError::LoginFail => (
            //         StatusCode::BAD_REQUEST,
            //         ClientError::AuthError(AuthError::LoginFail),
            //     ),
            //     AuthError::LogoutFail => (
            //         axum::http::StatusCode::BAD_REQUEST,
            //         ClientError::AuthError(AuthError::LogoutFail),
            //     ),
            // },
            JsonRejection => (StatusCode::BAD_REQUEST, ClientError::JsonRejection),
            EntityNotFound { entity } => (
                StatusCode::FORBIDDEN,
                ClientError::EntityNotFound {
                    entity: entity.clone(),
                },
            ),
            Validation(err) => (
                StatusCode::BAD_REQUEST,
                ClientError::Validation(err.clone()),
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServerError),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "message", content = "details", rename_all = "snake_case")]
pub enum ClientError {
    AuthError(AuthError),
    EntityNotFound { entity: String },
    JsonRejection,
    NotFound,
    ServerError,
    Validation(validator::ValidationErrors),
}
