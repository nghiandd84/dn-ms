use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use serde::Serialize;
use shared_shared_data_auth::{error::{AuthError, TokenError}, password::PasswordError};

#[derive(Debug)]
pub enum AppError {
    Auth(AuthError),
    Password(PasswordError),
    Token(TokenError),
    DbErr(sea_orm::DbErr),
    EntityNotFound { entity: String },
    JsonRejection,
    Unknown,
    Validation(validator::ValidationErrors),
}
// #[derive(Debug, Clone, Serialize)]
// #[serde(tag = "message", rename_all = "snake_case")]
// pub enum AuthError {
//     CtxNotInRequestExt,
//     LoginFail,
//     LogoutFail,
// }

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DbErr(err)
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
