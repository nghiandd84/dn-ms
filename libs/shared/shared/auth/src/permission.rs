use axum::{extract::FromRequestParts, http::request::Parts};
use shared_shared_data_error::{app::AppError, auth::AuthError};
use std::marker::PhantomData;
use tracing::debug;
use uuid::Uuid;

use crate::{claim::AccessTokenStruct, ResourcePermission};

pub const READ: u32 = 1 << 0; // 1
pub const CREATE: u32 = 1 << 1; // 2
pub const UPDATE: u32 = 1 << 2; // 4
pub const DELETE: u32 = 1 << 3; // 8
pub const ADMIN: u32 = 1 << 4; // 16 (The "Super User" bit)

pub struct Auth<R: ResourcePermission> {
    pub mask: u32,
    pub phantom_r: PhantomData<R>,
    pub user_id: Uuid,
    pub access_key: Option<String>,
}

impl<R> Auth<R>
where
    R: ResourcePermission,
{
    pub fn user_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }

    pub fn access_key(&self) -> Option<String> {
        self.access_key.clone()
    }
}

pub trait StatePermission {
    fn get_permission_map(&self, role_name: String, resource_name: String) -> u32;
    fn pull_permission(&self) -> impl std::future::Future<Output = Result<(), AuthError>>;
}

impl<S, R> FromRequestParts<S> for Auth<R>
where
    S: Send + Sync + StatePermission,
    R: ResourcePermission,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let baggage = parts
            .headers
            .get("baggage")
            // Exampe baggae header: accesses=BAKERY_SUPPORT*A_ACCESS_KEY|EMAIL_NOTIFICATION_SALE*B_ACCESS_KEY|SUPPORT*,user_id=066df7b0-dcd1-4e7c-94a1-9b5f68794ca7,client_id=123e4567-e89b-12d3-a456-426614174000
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Auth(AuthError::InsufficientPermission))?;
        debug!("baggage data {}", baggage);
        let access_token = AccessTokenStruct::from_string(baggage)
            .ok_or(AppError::Auth(AuthError::InsufficientPermission))?;
        debug!(
            "user_id {}, client_id {}",
            access_token.user_id, access_token.client_id
        );

        // Find the first access that has the required permission
        let (user_mask, access_key) = access_token
            .accesses
            .iter()
            .find_map(|access| {
                let resource_mask =
                    state.get_permission_map(access.role_name.clone(), R::RESOURCE.to_string());
                let is_admin = (resource_mask & ADMIN) == ADMIN;
                let has_perm = (resource_mask & R::BIT) == R::BIT;
                if is_admin || has_perm {
                    Some((resource_mask, access.key.clone()))
                } else {
                    None
                }
            })
            .ok_or(AppError::Auth(AuthError::InsufficientPermission))?;

        Ok(Auth {
            mask: user_mask,
            user_id: access_token.user_id, // Extracted from token claims
            access_key: access_key, // Placeholder, can be extracted from token claims if needed
            phantom_r: PhantomData,
        })
    }
}
