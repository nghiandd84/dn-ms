use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    claim::{Access, Claims, ClaimsSubject},
    data::AuthorizationCodeData,
    error::TokenError,
};

const TOKEN_TYPE: &str = "Bearer";
pub const TOKEN_EXPIRATION: i64 = 7200; // 2 hours in seconds
pub const REFRESH_TOKEN_EXPIRATION: i64 = 2592000; // 30 days in seconds

pub fn create_access_token(
    user_id: Uuid,
    client_secret: &str,
    accesses: Vec<Access>,
) -> Result<String, TokenError> {
    let now = Utc::now();

    let expiration = now + Duration::seconds(TOKEN_EXPIRATION);
    let claims = Claims {
        sub: ClaimsSubject {
            user_id: user_id,
            accesses: Some(accesses),
        },
        exp: expiration.timestamp() as u64,
        iat: now.timestamp() as u64,
    };

    // Using HS256 algorithm
    let header = Header::new(Algorithm::HS256);
    let access_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(client_secret.as_ref()),
    )
    .map_err(|_| TokenError::InvalidToken)?;

    // This function would typically create a JWT or similar token
    // For demonstration, we return a dummy token
    Ok(access_token)
}

pub fn create_refresh_token(user_id: Uuid, client_secret: &str) -> Result<String, TokenError> {
    let now = Utc::now();

    // Refresh token valid for 30 days
    let expiration = now + Duration::days(30);
    let claims = Claims {
        sub: ClaimsSubject {
            user_id: user_id,
            // Refresh token typically does not carry access rights
            accesses: None,
        },
        exp: expiration.timestamp() as u64,
        iat: now.timestamp() as u64,
    };

    // Using HS256 algorithm
    let header = Header::new(Algorithm::HS256);
    let refresh_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(client_secret.as_ref()),
    )
    .map_err(|_| TokenError::InvalidToken)?;

    // This function would typically create a JWT or similar token
    // For demonstration, we return a dummy token
    Ok(refresh_token)
}

pub fn create_authorization_data(
    user_id: Uuid,
    client_secret: &str,
    accesses: Vec<Access>,
    scopes: Vec<String>,
) -> Result<AuthorizationCodeData, TokenError> {
    let access_token = create_access_token(user_id, client_secret, accesses)
        .map_err(|_| TokenError::InvalidToken)?;
    let refresh_token =
        create_refresh_token(user_id, client_secret).map_err(|_| TokenError::InvalidToken)?;

    Ok(AuthorizationCodeData {
        access_token,
        token_type: TOKEN_TYPE.to_string(),
        expires_in: TOKEN_EXPIRATION,
        refresh_token: Some(refresh_token),
        refresh_expires_in: Some(REFRESH_TOKEN_EXPIRATION),
        scopes: Some(scopes), // Optional scope can be added if needed
        user_id,
    })
}
