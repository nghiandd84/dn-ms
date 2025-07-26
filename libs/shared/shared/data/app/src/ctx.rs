use axum::{extract::FromRequestParts, http::request::Parts};
use shared_shared_data_auth::error::AuthError;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<Ctx>() {
            Some(ctx) => Ok(ctx.clone()),
            None => Err(AppError::Auth(AuthError::Unknow)),
        }
    }
}
