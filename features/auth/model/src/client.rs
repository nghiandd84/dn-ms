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
        code = "client_key",
        message = "the length of client_key must be between 0 and 512"
    ))]
    pub client_key: Option<String>,

    #[validate(email(message = "email must be a valid email address"))]
    pub email: Option<String>,

    #[validate(length(
        min = 0,
        max = 512,
        code = "description",
        message = "the length of description must be between 0 and 512"
    ))]
    pub description: Option<String>,
    #[validate(length(
        min = 1,
        code = "redirect_uris",
        message = "redirect_uris must contain at least one URI"
    ))]
    pub redirect_uris: Vec<String>,
    #[validate(length(
        min = 1,
        code = "allowed_grants",
        message = "allowed_grants must contain at least one grant type"
    ))]
    pub allowed_grants: Vec<String>,
}

impl Into<ClientForCreateDto> for ClientForCreateRequest {
    fn into(self) -> ClientForCreateDto {
        ClientForCreateDto {
            name: self.name,
            description: self.description.unwrap_or_default(),
            client_key: self.client_key.unwrap_or_default(),
            email: self.email.unwrap_or_default(),
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
        code = "client_key",
        message = "the length of client_key must be between 0 and 512"
    ))]
    pub client_key: Option<String>,

    #[validate(length(
        min = 0,
        max = 512,
        code = "email",
        message = "the length of email must be between 0 and 512"
    ))]
    pub email: Option<String>,

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
            client_key: self.client_key,
            email: self.email,
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
    #[serde(skip_serializing)]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_grants: Option<VecString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uris: Option<VecString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl ClientData {
    pub fn get_email(&self) -> Option<String> {
        self.email.clone()
    }

    /// Return a new ClientData with only the selected fields populated.
    pub fn filter_fields(mut self, fields: &Vec<String>) -> Self {
        if !fields.contains(&"id".to_string()) {
            self.id = None;
        }
        if !fields.contains(&"name".to_string()) {
            self.name = None;
        }
        if !fields.contains(&"description".to_string()) {
            self.description = None;
        }
        if !fields.contains(&"email".to_string()) {
            self.email = None;
        }
        if !fields.contains(&"client_key".to_string()) {
            self.client_key = None;
        }
        if !fields.contains(&"redirect_uris".to_string()) {
            self.redirect_uris = None;
        }
        if !fields.contains(&"allowed_grants".to_string()) {
            self.allowed_grants = None;
        }
        self
    }
}

impl Into<ClientData> for ModelOptionDto {
    fn into(self) -> ClientData {
        ClientData {
            name: self.name,
            description: self.description,
            id: self.id,
            client_secret: self.client_secret,
            client_key: self.client_key,
            email: self.email,
            redirect_uris: self.redirect_uris,
            allowed_grants: self.allowed_grants,
            ..Default::default()
        }
    }
}

impl Into<ClientData> for features_auth_entities::client::Model {
    fn into(self) -> ClientData {
        ClientData {
            id: Some(self.id),
            name: Some(self.name),
            description: Some(self.description),
            client_secret: Some(self.client_secret),
            client_key: Some(self.client_key),
            email: Some(self.email),
            redirect_uris: Some(self.redirect_uris),
            allowed_grants: Some(self.allowed_grants),
        }
    }
}
