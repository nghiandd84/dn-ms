use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_translation_entities::translation_version::{
    ModelOptionDto, TranslationVersionForCreateDto, TranslationVersionForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TranslationVersionForCreateRequest {
    pub key_id: Uuid,
    #[validate(length(
        min = 2,
        max = 10,
        code = "locale",
        message = "the length of locale must be between 2 and 10"
    ))]
    pub locale: String,
    #[validate(length(min = 1, code = "content", message = "content cannot be empty"))]
    pub content: String,
    pub version_number: i32,
    #[validate(length(
        min = 1,
        max = 20,
        code = "status",
        message = "the length of status must be between 1 and 20"
    ))]
    pub status: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TranslationVersionForUpdateRequest {
    #[validate(length(min = 1, code = "content", message = "content cannot be empty"))]
    pub content: Option<String>,
    #[validate(length(
        min = 1,
        max = 20,
        code = "status",
        message = "the length of status must be between 1 and 20"
    ))]
    pub status: Option<String>,
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TranslationVersionData {
    pub id: Option<Uuid>,
    pub key_id: Option<Uuid>,
    pub locale: Option<String>,
    pub content: Option<String>,
    pub version_number: Option<i32>,
    pub status: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: Option<DateTime>,
}

impl Into<TranslationVersionData> for ModelOptionDto {
    fn into(self) -> TranslationVersionData {
        TranslationVersionData {
            id: self.id,
            key_id: self.key_id,
            locale: self.locale,
            content: self.content,
            version_number: self.version_number,
            status: self.status,
            created_by: self.created_by,
            created_at: self.created_at,
        }
    }
}

impl From<TranslationVersionForCreateRequest> for TranslationVersionForCreateDto {
    fn from(req: TranslationVersionForCreateRequest) -> Self {
        TranslationVersionForCreateDto {
            key_id: req.key_id,
            locale: req.locale,
            content: req.content,
            version_number: req.version_number,
            status: req.status.unwrap_or_else(|| "draft".to_string()),
            created_by: Uuid::nil(), // Placeholder, adjust as necessary
        }
    }
}

impl From<TranslationVersionForUpdateRequest> for TranslationVersionForUpdateDto {
    fn from(req: TranslationVersionForUpdateRequest) -> Self {
        TranslationVersionForUpdateDto {
            content: req.content,
            status: req.status,
        }
    }
}
