use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use shared_shared_macro::Response;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema, Response)]
pub struct AuthorizationCodeData {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub refresh_expires_in: Option<i64>,
    pub scopes: Option<Vec<String>>,
    pub user_id: Uuid,
}