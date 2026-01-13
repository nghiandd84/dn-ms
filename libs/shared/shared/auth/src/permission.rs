use axum::{extract::FromRequestParts, http::request::Parts};
use shared_shared_data_error::{app::AppError, auth::AuthError};
use std::marker::PhantomData;
use tracing::{debug, error};

use crate::ResourcePermission;

pub const READ: u32 = 1 << 0; // 1
pub const CREATE: u32 = 1 << 1; // 2
pub const UPDATE: u32 = 1 << 2; // 4
pub const DELETE: u32 = 1 << 3; // 8
pub const ADMIN: u32 = 1 << 4; // 16 (The "Super User" bit)




pub struct Auth<R: ResourcePermission> {
    pub mask: u32,
    phantom_r: PhantomData<R>,
}

impl<S, R> FromRequestParts<S> for Auth<R>
where
    S: Send + Sync,
    S: Send + Sync,
    R: ResourcePermission,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let baggage = parts
            .headers
            .get("baggage")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Auth(AuthError::InsufficientPermission))?;
        debug!("baggage data {}", baggage);

        // 1. Extract the access_key part
        let key_str = baggage
            .split(',')
            .find_map(|pair| {
                let mut kv = pair.splitn(2, '=');
                if kv.next()?.trim() == "accesses" {
                    kv.next()
                } else {
                    None
                }
            })
            .and_then(|val| val.split('*').nth(1))
            .ok_or(AppError::Auth(AuthError::InsufficientPermission))?;

        debug!("key {}", key_str);

        // 2. Parse the key as a Hex or Decimal number
        // Example: if key is "0x0F", parse it as hex
        let user_mask = u32::from_str_radix(key_str.trim_start_matches("0x"), 16)
            .map_err(|_| AppError::Auth(AuthError::InsufficientPermission))?;
        debug!("User mask {}", user_mask);
        // 3. Check against Resource Requirement
        let is_admin = (user_mask & ADMIN) == ADMIN;
        let has_perm = (user_mask & R::BIT) == R::BIT;
        if is_admin || has_perm {
            Ok(Auth {
                mask: user_mask,
                phantom_r: PhantomData,
            })
        } else {
            error!("Not have permission");
            Err(AppError::Auth(AuthError::InsufficientPermission))
        }
    }
}
