use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_wallet_entities::idempotency::{
    IdempotencyKeyForCreateDto, IdempotencyKeyForUpdateDto, ModelOptionDto,
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct IdempotencyKeyForCreateRequest {
    pub key: String,
    pub endpoint: String,
    pub request_hash: String,
    pub state: String,
    pub expires_at: Option<DateTime>,
    pub response_status: i32,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct IdempotencyKeyForUpdateRequest {
    pub response_body: Option<Json>,
    pub response_status: Option<i32>,
    pub state: Option<String>,
    pub expires_at: Option<DateTime>,
}

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct IdempotencyKeyData {
    pub id: Option<Uuid>,
    pub key: Option<String>,
    pub endpoint: Option<String>,
    pub request_hash: Option<String>,
    pub response_status: Option<i32>,
    pub state: Option<String>,
    pub created_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
}

impl Into<IdempotencyKeyData> for ModelOptionDto {
    fn into(self) -> IdempotencyKeyData {
        IdempotencyKeyData {
            id: self.id,
            key: self.key,
            endpoint: self.endpoint,
            request_hash: self.request_hash,
            response_status: self.response_status,
            state: self.state,
            created_at: self.created_at,
            expires_at: self.expires_at,
        }
    }
}

impl Into<IdempotencyKeyForCreateDto> for IdempotencyKeyForCreateRequest {
    fn into(self) -> IdempotencyKeyForCreateDto {
        IdempotencyKeyForCreateDto {
            key: self.key,
            endpoint: self.endpoint,
            request_hash: self.request_hash,
            state: self.state,
            expires_at: self.expires_at.unwrap(),
            response_status: self.response_status,
            ..Default::default() // response_body, response_status, expires_at will be set later
        }
    }
}

impl Into<IdempotencyKeyForUpdateDto> for IdempotencyKeyForUpdateRequest {
    fn into(self) -> IdempotencyKeyForUpdateDto {
        IdempotencyKeyForUpdateDto {
            response_status: self.response_status,
            state: self.state,
            expires_at: self.expires_at,
        }
    }
}
