use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_lookup_entities::lookup_item_translation::{
    LookupItemTranslationForCreateDto, LookupItemTranslationForUpdateDto, ModelOptionDto,
};

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct LookupItemTranslationData {
    pub id: Option<Uuid>,
    pub lookup_item_id: Option<Uuid>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<LookupItemTranslationData> for ModelOptionDto {
    fn into(self) -> LookupItemTranslationData {
        LookupItemTranslationData {
            id: self.id,
            lookup_item_id: self.lookup_item_id,
            locale: self.locale,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupItemTranslationForCreateRequest {
    #[validate(length(
        min = 1,
        max = 10,
        code = "translation_locale_length",
        message = "locale must be between 1 and 10 characters"
    ))]
    pub locale: String,
    #[validate(length(
        min = 1,
        max = 200,
        code = "translation_name_length",
        message = "name must be between 1 and 200 characters"
    ))]
    pub name: String,
    pub lookup_item_id: Uuid,
}

impl Into<LookupItemTranslationForCreateDto> for LookupItemTranslationForCreateRequest {
    fn into(self) -> LookupItemTranslationForCreateDto {
        LookupItemTranslationForCreateDto {
            locale: self.locale,
            name: self.name,
            lookup_item_id: self.lookup_item_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupItemTranslationForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 200,
        code = "translation_name_length",
        message = "name must be between 1 and 200 characters"
    ))]
    pub name: Option<String>,
}

impl Into<LookupItemTranslationForUpdateDto> for LookupItemTranslationForUpdateRequest {
    fn into(self) -> LookupItemTranslationForUpdateDto {
        LookupItemTranslationForUpdateDto { name: self.name }
    }
}
