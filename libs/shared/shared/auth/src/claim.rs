use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: ClaimsSubject, // Subject (User ID and accesses)
    pub exp: u64,           // Required: Expiration time
    pub iat: u64,           // Optional: Issued at time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsSubject {
    pub user_id: Uuid,
    pub accesses: Option<Vec<Access>>, // Optional: Access rights
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Access {
    pub role_name: String,
    pub key: Option<String>, // Optional: Access key
}
