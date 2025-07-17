use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub dn_data: ClaimSubject,
    // pub sub: ClaimSubject, // Subject (User ID and accesses)
    pub exp: u64,     // Required: Expiration time
    pub iat: u64,     // Optional: Issued at time
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // This tells Serde to try each variant until one matches
pub enum ClaimSubject {
    String(String),
    AccessToken(AccessTokenStruct),
    RefreshToken(RefreshTokenStruct),
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AccessTokenStruct {
    pub user_id: Uuid,
    pub accesses: Vec<Access>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenStruct {
    pub user_id: Uuid,
    pub token_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Access {
    pub role_name: String,
    pub key: Option<String>, // Optional: Access key
}
