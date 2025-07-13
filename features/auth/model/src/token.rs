use chrono::naive::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::token::{ModelOptionDto, TokenForCreateDto};
use shared_shared_macro::{ParamFilter, Response};
#[derive(Clone, Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TokenForCreateRequest {
    #[validate(required(code = "code_required", message = "code is required"))]
    pub code: Option<String>,
    #[validate(required(code = "client_id_required", message = "client id is required"))]
    pub client_id: Option<Uuid>,
    #[validate(required(code = "grant_type_required", message = "Grant type is required"))]
    pub grant_type: Option<GrantType>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}

impl Into<TokenForCreateDto> for TokenForCreateRequest {
    fn into(self) -> TokenForCreateDto {
        TokenForCreateDto {
            code: self.code,
            ..Default::default()
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TokenData {
    id: Option<Uuid>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<Uuid>,
    client_id: Option<Uuid>,
    scopes: Option<VecString>,
    access_token_expires_at: Option<DateTime>,
    refresh_token_expires_at: Option<DateTime>,
    revoked_at: Option<DateTime>,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
}

impl Into<TokenData> for ModelOptionDto {
    fn into(self) -> TokenData {
        TokenData {
            id: self.id,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            user_id: self.user_id,
            client_id: self.client_id,
            scopes: self.scopes.map(VecString::from),
            access_token_expires_at: self.access_token_expires_at,
            refresh_token_expires_at: self.refresh_token_expires_at,
            revoked_at: self.revoked_at.unwrap().or(None),
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}
