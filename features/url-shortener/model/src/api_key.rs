use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_url_shortener_entities::api_key::ModelOptionDto;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response)]
pub struct ApiKeyData {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub last_used_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

impl From<ModelOptionDto> for ApiKeyData {
    fn from(val: ModelOptionDto) -> Self {
        ApiKeyData {
            id: val.id,
            user_id: val.user_id,
            name: val.name,
            is_active: val.is_active,
            last_used_at: val.last_used_at.flatten(),
            created_at: val.created_at,
        }
    }
}

/// Response returned when a new API key is created.
/// The `key` field contains the plaintext key — shown only once.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct ApiKeyCreatedResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateApiKeyRequest {
    #[validate(length(
        min = 1,
        max = 100,
        code = "api_key_name_length",
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,
}
