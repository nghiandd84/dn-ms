use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_email_template_entities::template_placeholders::{
    ModelOptionDto, TemplatePlaceholderForCreateDto, TemplatePlaceholderForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TemplatePlaceholderForCreateRequest {
    #[validate(required(message = "template_id is required"))]
    pub template_id: Option<i32>,
    #[validate(length(
        min = 2,
        max = 100,
        code = "placeholder_keye_length",
        message = "the length of placeholder_key must be between 2 and 100"
    ))]
    pub placeholder_key: String,

    #[validate(length(
        min = 2,
        max = 255,
        code = "description_length",
        message = "the length of description must be between 2 and 255"
    ))]
    pub description: String,

    #[validate(length(
        min = 1,
        max = 255,
        code = "example_value_length",
        message = "the length of example_value must be between 1 and 255"
    ))]
    pub example_value: String,

    #[validate(required(message = "is_required is required"))]
    pub is_required: Option<bool>,
}

impl Into<TemplatePlaceholderForCreateDto> for TemplatePlaceholderForCreateRequest {
    fn into(self) -> TemplatePlaceholderForCreateDto {
        TemplatePlaceholderForCreateDto {
            template_id: self.template_id.unwrap(),
            placeholder_key: self.placeholder_key,
            description: self.description,
            example_value: self.example_value,
            is_required: self.is_required.unwrap_or(false),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TemplatePlaeholderForUpdateRequest {
    #[validate(length(
        min = 2,
        max = 100,
        code = "placeholder_keye_length",
        message = "the length of placeholder_key must be between 2 and 100"
    ))]
    pub placeholder_key: Option<String>,

    #[validate(length(
        min = 2,
        max = 255,
        code = "description_length",
        message = "the length of description must be between 2 and 255"
    ))]
    pub description: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        code = "example_value_length",
        message = "the length of example_value must be between 1 and 255"
    ))]
    pub example_value: Option<String>,
    pub is_required: Option<bool>,
}

impl Into<TemplatePlaceholderForUpdateDto> for TemplatePlaeholderForUpdateRequest {
    fn into(self) -> TemplatePlaceholderForUpdateDto {
        TemplatePlaceholderForUpdateDto {
            placeholder_key: self.placeholder_key,
            description: self.description,
            example_value: self.example_value,
            is_required: self.is_required,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TemplatePlaceholderData {
    id: Option<i32>,
    template_id: Option<i32>,
    placeholder_key: Option<String>,
    description: Option<String>,
    example_value: Option<String>,
    is_required: Option<bool>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl Into<TemplatePlaceholderData> for ModelOptionDto {
    fn into(self) -> TemplatePlaceholderData {
        TemplatePlaceholderData {
            id: self.id,
            template_id: self.template_id,
            placeholder_key: self.placeholder_key,
            description: self.description,
            example_value: self.example_value,
            is_required: self.is_required,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}
