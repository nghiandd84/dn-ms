use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_merchant_entities::webhook::{
    ModelOptionDto, WebhookForCreateDto, WebhookForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct WebhookData {
    pub id: Option<Uuid>,
    pub merchant_id: Option<String>,
    pub url: Option<String>,
    pub event_types: Option<VecString>,
    pub secret: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<WebhookData> for ModelOptionDto {
    fn into(self) -> WebhookData {
        WebhookData {
            id: self.id,
            merchant_id: self.merchant_id,
            url: self.url,
            event_types: self.event_types,
            secret: self.secret,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebhookForCreateRequest {
    #[validate(length(min = 1, message = "merchant_id is required"))]
    pub merchant_id: String,
    #[validate(url(message = "url must be a valid URL"))]
    pub url: String,
    #[validate(length(min = 1, message = "event_types is required"))]
    pub event_types: Vec<String>,
    #[validate(length(min = 1, message = "secret is required"))]
    pub secret: String,
    #[validate(length(min = 1, message = "status is required"))]
    pub status: String,
}

impl Into<WebhookForCreateDto> for WebhookForCreateRequest {
    fn into(self) -> WebhookForCreateDto {
        WebhookForCreateDto {
            merchant_id: self.merchant_id,
            url: self.url,
            event_types: self.event_types,
            secret: self.secret,
            status: self.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebhookForUpdateRequest {
    #[validate(url(message = "url must be a valid URL"))]
    pub url: Option<String>,
    pub event_types: Option<VecString>,
    pub secret: Option<String>,
    pub status: Option<String>,
}

impl Into<WebhookForUpdateDto> for WebhookForUpdateRequest {
    fn into(self) -> WebhookForUpdateDto {
        WebhookForUpdateDto {
            url: self.url,
            event_types: self.event_types,
            secret: self.secret,
            status: self.status,
        }
    }
}
