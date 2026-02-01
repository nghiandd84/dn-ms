use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_translation_entities::translation_key::{
    ModelOptionDto, TranslationKeyForCreateDto, TranslationKeyForUpdateDto
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TranslationKeyForCreateRequest {
    pub project_id: Uuid,
    #[validate(length(
        min = 1,
        max = 255,
        code = "key_name",
        message = "the length of key_name must be between 1 and 255"
    ))]
    pub key_name: String,
    #[validate(length(
        min = 0,
        max = 2000,
        code = "description",
        message = "the length of description must not exceed 2000 characters"
    ))]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TranslationKeyForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "key_name",
        message = "the length of key_name must be between 1 and 255"
    ))]
    pub key_name: Option<String>,
    #[validate(length(
        min = 0,
        max = 2000,
        code = "description",
        message = "the length of description must not exceed 2000 characters"
    ))]
    pub description: Option<String>,
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TranslationKeyData {
    pub id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub key_name: Option<String>,
    pub description: Option<String>,
}

impl Into<TranslationKeyData> for ModelOptionDto {
    fn into(self) -> TranslationKeyData {
        TranslationKeyData {
            id: self.id,
            project_id: self.project_id,
            key_name: self.key_name,
            description: self.description,
        }
    }
}

impl From<TranslationKeyForCreateRequest> for TranslationKeyForCreateDto {
    fn from(req: TranslationKeyForCreateRequest) -> Self {
        TranslationKeyForCreateDto {
            project_id: req.project_id,
            user_id: Uuid::nil(), // Placeholder, adjust as necessary
            key_name: req.key_name,
            description: req.description.unwrap_or_default(),
        }
    }
}

impl From<TranslationKeyForUpdateRequest> for TranslationKeyForUpdateDto {
    fn from(req: TranslationKeyForUpdateRequest) -> Self {
        TranslationKeyForUpdateDto {
            key_name: req.key_name,
            description: req.description,
        }
    }
}
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct AssignTagsRequest {
    #[validate(length(min = 1, code = "tag_ids", message = "tag_ids must contain at least one tag"))]
    pub tag_ids: Vec<Uuid>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct UnassignTagsRequest {
    #[validate(length(min = 1, code = "tag_ids", message = "tag_ids must contain at least one tag"))]
    pub tag_ids: Vec<Uuid>,
}