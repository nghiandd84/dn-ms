use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_translation_entities::project::{
    ModelOptionDto, ProjectForCreateDto, ProjectForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ProjectForCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "name",
        message = "the length of name must be between 1 and 255"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 64,
        code = "api_key",
        message = "the length of api_key must be between 1 and 64"
    ))]
    pub api_key: String,
    #[validate(length(
        min = 2,
        max = 10,
        code = "default_locale",
        message = "the length of default_locale must be between 2 and 10"
    ))]
    pub default_locale: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ProjectForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "name",
        message = "the length of name must be between 1 and 255"
    ))]
    pub name: Option<String>,
    #[validate(length(
        min = 2,
        max = 10,
        code = "default_locale",
        message = "the length of default_locale must be between 2 and 10"
    ))]
    pub default_locale: Option<String>,
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Response, ParamFilter)]
pub struct ProjectData {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub default_locale: Option<String>,
    pub created_at: Option<DateTime>,
}

impl Into<ProjectData> for ModelOptionDto {
    fn into(self) -> ProjectData {
        ProjectData {
            id: self.id,
            name: self.name,
            api_key: self.api_key,
            default_locale: self.default_locale,
            created_at: self.created_at,
        }
    }
}

impl From<ProjectForCreateRequest> for ProjectForCreateDto {
    fn from(req: ProjectForCreateRequest) -> Self {
        ProjectForCreateDto {
            name: req.name,
            api_key: req.api_key,
            user_id: Uuid::new_v4(), // Placeholder, replace with actual user ID logic
            default_locale: req.default_locale.unwrap_or_else(|| "en-US".to_string()),
        }
    }
}

impl From<ProjectForUpdateRequest> for ProjectForUpdateDto {
    fn from(req: ProjectForUpdateRequest) -> Self {
        ProjectForUpdateDto {
            name: req.name,
            default_locale: req.default_locale,
        }
    }
}
