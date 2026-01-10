use axum::{extract::FromRequestParts, http::request::Parts};
use shared_shared_data_error::{app::AppError, auth::AuthError};
use std::marker::PhantomData;
use tracing::{debug, error};

const READ: u32 = 1 << 0; // 1
const CREATE: u32 = 1 << 1; // 2
const UPDATE: u32 = 1 << 2; // 4
const DELETE: u32 = 1 << 3; // 8
const ADMIN: u32 = 1 << 4; // 16 (The "Super User" bit)

pub trait BitmaskRequirement {
    const REQUIRED_BIT: u32;
}

pub struct RequireRead;
impl BitmaskRequirement for RequireRead {
    const REQUIRED_BIT: u32 = READ;
}

pub struct RequireCreate;
impl BitmaskRequirement for RequireCreate {
    const REQUIRED_BIT: u32 = CREATE;
}

pub struct RequireUpdate;
impl BitmaskRequirement for RequireUpdate {
    const REQUIRED_BIT: u32 = UPDATE;
}

pub struct RequireDelete;
impl BitmaskRequirement for RequireDelete {
    const REQUIRED_BIT: u32 = DELETE;
}

// Requirement: Must have both READ and WRITE bits
pub struct RequireReadUpdate;
impl BitmaskRequirement for RequireReadUpdate {
    const REQUIRED_BIT: u32 = READ | UPDATE;
}

pub struct RequireAdmin;
impl BitmaskRequirement for RequireAdmin {
    const REQUIRED_BIT: u32 = ADMIN;
}

pub struct Auth<R: BitmaskRequirement> {
    pub mask: u32,
    phantom_r: PhantomData<R>,
}

impl<S, R> FromRequestParts<S> for Auth<R>
where
    S: Send + Sync,
    R: BitmaskRequirement,
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
        // 3. Bitwise  check
        let is_admin = (user_mask & ADMIN) == ADMIN;
        let has_required = (user_mask & R::REQUIRED_BIT) == R::REQUIRED_BIT;
        if is_admin || has_required {
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
