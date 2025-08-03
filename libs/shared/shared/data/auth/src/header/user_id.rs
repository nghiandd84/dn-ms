use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::header::error::HeaderError; // For sharing AppState across threads

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

// Implement the FromRequestParts trait for our UserId struct, now generic over the state `S`.

impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
    // or more simply, if S *is* AppState, this is satisfied.
{
    type Rejection = HeaderError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to get the 'dn_user_id' header from the request parts.
        let header_value = parts.headers.get("dn_user_id");

        let user_id_str = match header_value {
            Some(value) => {
                let s = value.to_str().map_err(|_| HeaderError::MissingHeader)?;
                if s.is_empty() {
                    return Err(HeaderError::EmptyHeader);
                }
                s.to_string()
            }
            None => {
                return Err(HeaderError::MissingHeader);
            }
        };

        Ok(UserId(
            Uuid::parse_str(&user_id_str).map_err(|_| HeaderError::Unauthorized)?,
        ))
    }
}
