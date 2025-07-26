use std::str::FromStr;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::debug;
use uuid::Uuid;

use crate::claim::{AccessTokenStruct, ClaimSubject, Claims, RefreshTokenStruct, UserAccessData};

use shared_shared_data_auth::error::TokenError;

pub const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRATION: i64 = 7200; // 2 hours in seconds
pub const REFRESH_TOKEN_EXPIRATION: i64 = 2592000; // 30 days in seconds

pub fn get_access_token_cache_key(user_id: Uuid) -> String {
    format!("{}:A", user_id)
}

pub fn get_refresh_token_cache_key(user_id: Uuid) -> String {
    format!("{}:R", user_id)
}

pub fn create_access_token(
    user_id: Uuid,
    client_secret: &str,
    accesses: Vec<UserAccessData>,
) -> Result<(String, Uuid), TokenError> {
    let now = Utc::now();
    let jti = Uuid::new_v4();

    let expiration = now + Duration::seconds(TOKEN_EXPIRATION);

    let claims = Claims {
        dn_data: ClaimSubject::AccessToken(AccessTokenStruct { user_id, accesses }),
        exp: expiration.timestamp() as u64,
        iat: now.timestamp() as u64,
        jti: jti.clone().to_string(),
    };

    // Using HS256 algorithm
    let header = Header::new(Algorithm::HS256);
    let access_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(client_secret.as_ref()),
    )
    .map_err(|error| {
        debug!("Failed to create token: {}", error);
        TokenError::CanNotCreateToken
    })?;
    Ok((access_token, jti))
}

pub fn create_refresh_token(
    user_id: Uuid,
    client_secret: &str,
    token_id: Uuid,
) -> Result<(String, Uuid), TokenError> {
    let now = Utc::now();
    let jti: Uuid = Uuid::new_v4();
    // Refresh token valid for 30 days
    let expiration = now + Duration::seconds(REFRESH_TOKEN_EXPIRATION);
    let claims = Claims {
        dn_data: ClaimSubject::RefreshToken(RefreshTokenStruct { user_id, token_id }),
        exp: expiration.timestamp() as u64,
        iat: now.timestamp() as u64,
        jti: jti.to_string(),
    };

    // Using HS256 algorithm
    let header = Header::new(Algorithm::HS256);
    let refresh_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(client_secret.as_ref()),
    )
    .map_err(|error| {
        debug!("Failed to create refresh token: {}", error);
        TokenError::CanNotCreateToken
    })?;

    Ok((refresh_token, jti))
}

pub fn decode_access_token(
    refresh_token: &str,
    client_secret: &str,
) -> Result<AccessTokenStruct, TokenError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_nbf = true;
    validation.reject_tokens_expiring_in_less_than = 10; // seconds
    let data = decode::<Claims>(
        refresh_token,
        &DecodingKey::from_secret(client_secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims.dn_data)
    .map_err(|error| {
        debug!("Failed to decode refresh token: {}", error);
        TokenError::InvalidToken
    })?;

    match data {
        ClaimSubject::AccessToken(token_data) => Ok(token_data),
        _ => Err(TokenError::InvalidToken),
    }
}

pub fn decode_refresh_token(
    refresh_token: &str,
    client_secret: &str,
) -> Result<(RefreshTokenStruct, Uuid), TokenError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_nbf = true;
    validation.reject_tokens_expiring_in_less_than = 10; // seconds
    let data = decode::<Claims>(
        refresh_token,
        &DecodingKey::from_secret(client_secret.as_ref()),
        &validation,
    )
    .map_err(|error| {
        debug!("Failed to decode refresh token: {}", error);
        TokenError::InvalidToken
    })?;
    let jti = Uuid::from_str(data.claims.jti.as_str()).unwrap();

    match data.claims.dn_data {
        ClaimSubject::RefreshToken(refresh_data) => Ok((refresh_data, jti)),
        _ => Err(TokenError::InvalidToken),
    }
}
