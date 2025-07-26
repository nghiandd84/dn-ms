use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::auth_code::{AuthCodeForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct AuthCodeForCreateRequest {
    #[validate(required(code = "user_id_required", message = "user_id is required"))]
    pub user_id: Option<Uuid>,
    #[validate(required(code = "client_id_required", message = "client_id is required"))]
    pub client_id: Option<Uuid>,
    #[validate(required(code = "scopes_required", message = "scopes is required"))]
    #[validate(length(
        min = 1,
        code = "scopes_length",
        message = "scopes must contain at least one item"
    ))]
    pub scopes: Option<Vec<String>>,
    #[validate(length(
        min = 1,
        max = 1204,
        code = "redirect_uri",
        message = "the length of redirect_uri must be between 1 and 1204"
    ))]
    pub redirect_uri: Option<String>,
}

impl Into<AuthCodeForCreateDto> for AuthCodeForCreateRequest {
    fn into(self) -> AuthCodeForCreateDto {
        AuthCodeForCreateDto {
            user_id: self.user_id.unwrap_or_default(),
            client_id: self.client_id.unwrap_or_default(),
            scopes: self.scopes.unwrap_or_default(),
            redirect_uri: self.redirect_uri.unwrap_or_default(),
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct AuthCodeData {
    pub user_id: Option<Uuid>,
    pub scopes: Option<VecString>,
    pub client_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub code: Option<String>,
    redirect_uri: Option<String>,
    expires_at: Option<DateTime>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl Into<AuthCodeData> for ModelOptionDto {
    fn into(self) -> AuthCodeData {
        AuthCodeData {
            id: self.id,
            code: self.code,
            user_id: self.user_id,
            client_id: self.client_id,
            scopes: self.scopes,
            redirect_uri: self.redirect_uri,
            expires_at: self.expires_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}
