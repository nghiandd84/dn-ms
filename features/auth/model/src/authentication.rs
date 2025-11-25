use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::authentication::{AuthenticationRequestForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct AuthenticationCreateRequest {
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

    #[validate(length(
        min = 1,
        max = 1204,
        code = "redirect_uri",
        message = "the length of response_type must be between 1 and 1204"
    ))]
    pub response_type: Option<String>,

    #[validate(length(
        min = 1,
        max = 1204,
        code = "state",
        message = "the length of state must be between 1 and 6020"
    ))]
    pub state: Option<String>,
}

impl Into<AuthenticationRequestForCreateDto> for AuthenticationCreateRequest {
    fn into(self) -> AuthenticationRequestForCreateDto {
        AuthenticationRequestForCreateDto {
            client_id: self.client_id.unwrap_or_default(),
            scopes: self.scopes.unwrap_or_default(),
            response_type: self.response_type.unwrap_or_default(),
            state: self.state.unwrap_or_default(),
            redirect_uri: self.redirect_uri.unwrap_or_default(),
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct AuthenticationRequestData {
    pub client_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub response_type: Option<String>,
    pub scopes: Option<VecString>,
    pub state: Option<String>,
    pub redirect_uri: Option<String>,
    expires_at: Option<DateTime>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl Into<AuthenticationRequestData> for ModelOptionDto {
    fn into(self) -> AuthenticationRequestData {
        AuthenticationRequestData {
            id: self.id,
            client_id: self.client_id,
            scopes: self.scopes,
            state: self.state,
            response_type: self.response_type,
            redirect_uri: self.redirect_uri,
            expires_at: self.expires_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, Validate)]
pub struct AuthLoginRequest {
    #[validate(required(code = "email_required", message = "email is required"))]
    pub email: Option<String>,
    #[validate(required(code = "password_required", message = "password is required"))]
    pub password: Option<String>,
    #[validate(required(code = "state_required", message = "state is required"))]
    pub state: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, ToSchema, Response, Debug)]
pub struct AuthLoginData {
    pub id_token: String,
    pub redirect_uri: String,
}
