use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_macro::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub dn_data: ClaimSubject,
    pub exp: u64, // Required: Expiration time
    pub iat: u64, // Optional: Issued at time
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // This tells Serde to try each variant until one matches
pub enum ClaimSubject {
    String(String),
    AccessToken(AccessTokenStruct),
    RefreshToken(RefreshTokenStruct),
}
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema, Response)]
pub struct AccessTokenStruct {
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub accesses: Vec<UserAccessData>,
}

impl AccessTokenStruct {
    pub fn access_to_string(&self) -> String {
        let access_str = self
            .accesses
            .iter()
            .map(|access| access.to_string())
            .collect::<Vec<_>>()
            .join("|");
        access_str
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenStruct {
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub token_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct UserAccessData {
    pub role_name: String,
    pub key: Option<String>, // Optional: Access key
}

impl ToString for UserAccessData {
    fn to_string(&self) -> String {
        match &self.key {
            Some(key) => format!("{}*{}", self.role_name, key),
            None => format!("{}*", self.role_name),
        }
    }
}
