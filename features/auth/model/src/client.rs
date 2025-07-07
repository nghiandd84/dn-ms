use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::client::{ClientForCreateDto, ClientForUpdateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ClientForCreateRequest {
    #[validate(length(
        min = 10,
        max = 128,
        code = "client_secret",
        message = "the length of client_secret must be between 10 and 128"
    ))]
    pub client_secret: String,
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
    pub redirect_uris: Vec<String>,
    pub allowed_grants: Vec<String>,
}

impl Into<ClientForCreateDto> for ClientForCreateRequest {
    fn into(self) -> ClientForCreateDto {
        ClientForCreateDto {
            name: self.name,
            description: self.description.unwrap_or_default(),
            redirect_uris: self.redirect_uris,
            allowed_grants: self.allowed_grants,
            client_secret: self.client_secret,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ClientForUpdateRequest {
    #[validate(length(
        min = 10,
        max = 128,
        code = "client_secret",
        message = "the length of client_secret must be between 10 and 128"
    ))]
    pub client_secret: Option<String>,
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
    pub redirect_uris: Option<Vec<String>>,
    pub allowed_grants: Option<Vec<String>>,
}

impl Into<ClientForUpdateDto> for ClientForUpdateRequest {
    fn into(self) -> ClientForUpdateDto {
        ClientForUpdateDto {
            description: self.description,
            name: self.name,
            client_secret: self.client_secret,
            redirect_uris: self.redirect_uris,
            allowed_grants: self.allowed_grants,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct ClientData {
    id: Option<Uuid>,
    client_secret: Option<String>,
    name: Option<String>,
    description: Option<String>,
    redirect_uris: Option<VecString>,
    allowed_grants: Option<VecString>,
}

impl Into<ClientData> for ModelOptionDto {
    fn into(self) -> ClientData {
        ClientData {
            name: self.name,
            description: self.description,
            id: self.id,
            client_secret: self.client_secret,
            redirect_uris: self.redirect_uris,
            allowed_grants: self.allowed_grants,
            ..Default::default()
        }
    }
}
