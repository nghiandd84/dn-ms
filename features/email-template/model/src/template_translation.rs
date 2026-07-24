use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
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

    #[schema(ignore)]
    pub user_id: Option<Uuid>,
}

impl Into<TemplateTranslationForCreateDto> for TemplateTranslationForCreateRequest {
    fn into(self) -> TemplateTranslationForCreateDto {
        TemplateTranslationForCreateDto {
            template_id: self.template_id.unwrap(),
            language_code: self.language_code,
            subject: self.subject,
            body: self.body,
            version_name: self.version_name,
            user_id: self.user_id.unwrap(),
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

use crate::email_template::{EmailTemplateData, EmailTemplateDataFilterParams};

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Deserialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TemplateTranslationData {
    id: Option<i32>,
    template_id: Option<i32>,
    language_code: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    version_name: Option<String>,
    user_id: Option<Uuid>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub email_template: Option<EmailTemplateData>,
}

impl TemplateTranslationData {
    pub fn get_subject(&self) -> String {
        self.subject.clone().unwrap_or_default()
    }
    pub fn get_body(&self) -> String {
        self.body.clone().unwrap_or_default()
    }
    pub fn get_template_id(&self) -> Option<i32> {
        self.template_id
    }
    pub fn set_email_template(&mut self, template: EmailTemplateData) {
        self.email_template = Some(template);
    }
}

impl Into<TemplateTranslationData> for ModelOptionDto {
    fn into(self) -> TemplateTranslationData {
        let email_template_data: Option<EmailTemplateData> = self
            .email_templates
            .and_then(|et| et.into_iter().next().map(|m| m.into()));

        TemplateTranslationData {
            id: self.id,
            template_id: self.template_id,
            language_code: self.language_code,
            subject: self.subject,
            body: self.body,
            version_name: self.version_name,
            user_id: self.user_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            email_template: email_template_data,
            ..Default::default()
        }
    }
}
