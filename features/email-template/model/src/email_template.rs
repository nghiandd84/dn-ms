use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_email_template_entities::email_templates::{
    EmailTemplateForCreateDto, EmailTemplateForUpdateDto, ModelOptionDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct EmailTemplateForCreateRequest {
    #[validate(length(
        min = 2,
        max = 128,
        code = "name",
        message = "the length of name must be between 2 and 128"
    ))]
    pub name: String,
    #[validate(length(
        min = 0,
        max = 512,
        code = "description",
        message = "the length of description must be between 0 and 512"
    ))]
    pub description: Option<String>,
}

impl Into<EmailTemplateForCreateDto> for EmailTemplateForCreateRequest {
    fn into(self) -> EmailTemplateForCreateDto {
        EmailTemplateForCreateDto {
            name: self.name,
            description: self.description.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct EmailTemplateForUpdateRequest {
    #[validate(length(
        min = 2,
        max = 128,
        code = "name",
        message = "the length of name must be between 2 and 128"
    ))]
    pub name: Option<String>,
    #[validate(length(
        min = 0,
        max = 512,
        code = "description",
        message = "the length of description must be between 0 and 512"
    ))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

impl Into<EmailTemplateForUpdateDto> for EmailTemplateForUpdateRequest {
    fn into(self) -> EmailTemplateForUpdateDto {
        EmailTemplateForUpdateDto {
            name: self.name,
            description: self.description,
            is_active: self.is_active,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct EmailTemplateData {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    is_active: Option<bool>,
    user_id: Option<Uuid>,
}

impl Into<EmailTemplateData> for ModelOptionDto {
    fn into(self) -> EmailTemplateData {
        EmailTemplateData {
            name: self.name,
            description: self.description,
            id: self.id,
            is_active: self.is_active,
            user_id: self.user_id,
            ..Default::default()
        }
    }
}
