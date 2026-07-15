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

use features_url_shortener_entities::shortened_url::{
    ModelOptionDto, ShortenedUrlForCreateDto, ShortenedUrlForUpdateDto,
};

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct ShortenedUrlData {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub original_url: Option<String>,
    pub short_code: Option<String>,
    pub title: Option<String>,
    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime>,
    pub click_count: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for ShortenedUrlData {
    fn from(val: ModelOptionDto) -> Self {
        ShortenedUrlData {
            id: val.id,
            user_id: val.user_id,
            original_url: val.original_url,
            short_code: val.short_code,
            title: val.title,
            is_active: val.is_active,
            expires_at: val.expires_at.flatten(),
            click_count: val.click_count,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateShortenedUrlRequest {
    #[validate(length(
        min = 1,
        max = 2048,
        code = "original_url_length",
        message = "URL must be between 1 and 2048 characters"
    ))]
    #[validate(url(code = "original_url_format", message = "Must be a valid URL"))]
    pub original_url: String,

    #[validate(length(
        min = 3,
        max = 30,
        code = "custom_code_length",
        message = "Custom code must be between 3 and 30 characters"
    ))]
    pub custom_code: Option<String>,

    #[validate(length(
        max = 255,
        code = "title_length",
        message = "Title must not exceed 255 characters"
    ))]
    pub title: Option<String>,

    pub expires_at: Option<DateTime>,

    #[serde(skip)]
    pub user_id: Option<Uuid>,
}

impl CreateShortenedUrlRequest {
    pub fn into_dto(self, short_code: String) -> ShortenedUrlForCreateDto {
        ShortenedUrlForCreateDto {
            user_id: self.user_id.unwrap_or_default(),
            original_url: self.original_url,
            short_code,
            title: self.title.unwrap_or_default(),
            expires_at: self.expires_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateShortenedUrlRequest {
    #[validate(length(
        max = 255,
        code = "title_length",
        message = "Title must not exceed 255 characters"
    ))]
    pub title: Option<String>,
    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime>,
}

impl From<UpdateShortenedUrlRequest> for ShortenedUrlForUpdateDto {
    fn from(val: UpdateShortenedUrlRequest) -> Self {
        ShortenedUrlForUpdateDto {
            title: val.title,
            is_active: val.is_active,
            expires_at: Some(val.expires_at),
        }
    }
}
