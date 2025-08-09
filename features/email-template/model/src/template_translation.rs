use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_email_template_entities::template_translations::{
    ModelOptionDto, TemplateTranslationForCreateDto, TemplateTranslationForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TemplateTranslationForCreateRequest {
    #[validate(required(message = "template_id is required"))]
    pub template_id: Option<i32>,
    #[validate(length(
        min = 2,
        max = 10,
        code = "language_code_length",
        message = "the length of name must be between 2 and 10"
    ))]
    pub language_code: String,

    #[validate(length(
        min = 2,
        max = 255,
        code = "subject_length",
        message = "the length of name must be between 2 and 255"
    ))]
    pub subject: String,

    #[validate(length(
        min = 2,
        code = "body_length",
        message = "the length of description must be greater than 2 "
    ))]
    pub body: String,

    #[validate(length(
        min = 2,
        max = 50,
        code = "version_name_length",
        message = "the length of name must be between 2 and 50"
    ))]
    pub version_name: String,
}

impl Into<TemplateTranslationForCreateDto> for TemplateTranslationForCreateRequest {
    fn into(self) -> TemplateTranslationForCreateDto {
        TemplateTranslationForCreateDto {
            template_id: self.template_id.unwrap(),
            language_code: self.language_code,
            subject: self.subject,
            body: self.body,
            version_name: self.version_name,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TemplateTranslationForUpdateRequest {
    pub template_id: Option<i32>,
    #[validate(length(
        min = 2,
        max = 10,
        code = "language_code_length",
        message = "the length of name must be between 2 and 10"
    ))]
    pub language_code: Option<String>,

    #[validate(length(
        min = 2,
        max = 255,
        code = "subject_length",
        message = "the length of name must be between 2 and 255"
    ))]
    pub subject: Option<String>,

    #[validate(length(
        min = 2,
        code = "body_length",
        message = "the length of description must be greater than 2 "
    ))]
    pub body: Option<String>,

    #[validate(length(
        min = 2,
        max = 50,
        code = "version_name_length",
        message = "the length of name must be between 2 and 50"
    ))]
    pub version_name: Option<String>,
}

impl Into<TemplateTranslationForUpdateDto> for TemplateTranslationForUpdateRequest {
    fn into(self) -> TemplateTranslationForUpdateDto {
        TemplateTranslationForUpdateDto {
            language_code: self.language_code,
            subject: self.subject,
            body: self.body,
            version_name: self.version_name,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TemplateTranslationData {
    id: Option<i32>,
    template_id: Option<i32>,
    language_code: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    version_name: Option<String>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl Into<TemplateTranslationData> for ModelOptionDto {
    fn into(self) -> TemplateTranslationData {
        TemplateTranslationData {
            id: self.id,
            template_id: self.template_id,
            language_code: self.language_code,
            subject: self.subject,
            body: self.body,
            version_name: self.version_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}
