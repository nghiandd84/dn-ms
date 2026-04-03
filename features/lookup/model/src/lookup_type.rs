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

use features_lookup_entities::lookup_type::{
    LookupTypeForCreateDto, LookupTypeForUpdateDto, ModelOptionDto,
};

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct LookupTypeData {
    pub id: Option<Uuid>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<LookupTypeData> for ModelOptionDto {
    fn into(self) -> LookupTypeData {
        LookupTypeData {
            id: self.id,
            code: self.code,
            name: self.name,
            description: self.description,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupTypeForCreateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "lookup_type_code_length",
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 100,
        code = "lookup_type_name_length",
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(
        max = 500,
        code = "lookup_type_description_length",
        message = "description must not exceed 500 characters"
    ))]
    pub description: Option<String>,
}

impl Into<LookupTypeForCreateDto> for LookupTypeForCreateRequest {
    fn into(self) -> LookupTypeForCreateDto {
        LookupTypeForCreateDto {
            code: self.code,
            name: self.name,
            description: self.description.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupTypeForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "lookup_type_code_length",
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: Option<String>,
    #[validate(length(
        min = 1,
        max = 100,
        code = "lookup_type_name_length",
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(
        max = 500,
        code = "lookup_type_description_length",
        message = "description must not exceed 500 characters"
    ))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

impl Into<LookupTypeForUpdateDto> for LookupTypeForUpdateRequest {
    fn into(self) -> LookupTypeForUpdateDto {
        LookupTypeForUpdateDto {
            code: self.code,
            name: self.name,
            description: self.description,
            is_active: self.is_active,
        }
    }
}
