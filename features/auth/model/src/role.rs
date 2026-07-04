use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::role::{ModelOptionDto, RoleForCreateDto, RoleForUpdateDto};
use shared_shared_macro::{ParamFilter, Response};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RoleForCreateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "name_length",
        message = "the length of email must be between 1 and 50"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 250,
        code = "description_length",
        message = "the length of first_name must be between 1 and 250"
    ))]
    pub description: String,

    #[validate(required(code = "client_id_required", message = "client_id is required"))]
    pub client_id: Option<Uuid>,
    pub is_default: Option<bool>,
}

impl Into<RoleForCreateDto> for RoleForCreateRequest {
    fn into(self) -> RoleForCreateDto {
        RoleForCreateDto {
            name: self.name,
            description: self.description,
            client_id: self.client_id.unwrap_or_default(),
            is_default: self.is_default.unwrap_or(false),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RoleForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "name_length",
        message = "the length of name must be between 1 and 50"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 250,
        code = "description_length",
        message = "the length of description must be between 1 and 250"
    ))]
    pub description: String,

    #[validate(required(code = "client_id_required", message = "client_id is required"))]
    pub client_id: Option<Uuid>,
    pub is_default: Option<bool>,
}

impl Into<RoleForUpdateDto> for RoleForUpdateRequest {
    fn into(self) -> RoleForUpdateDto {
        RoleForUpdateDto {
            name: Some(self.name),
            description: Some(self.description),
            client_id: self.client_id,
            is_default: self.is_default,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct AssignPermissionToRoleRequest {
    pub permission_ids: Vec<Uuid>,
}

use crate::client::ClientData;
use crate::permission::PermissionData;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
    query_params::QueryParams,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct RoleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,

    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionData>>,

    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientData>,
}

impl RoleData {
    pub fn get_id(&self) -> Option<Uuid> {
        self.id
    }

    /// Apply field selection from query params.
    /// - `?fields=id,name` filters top-level entity fields
    /// - `?includes=client[id,name]` filters fields within related entities
    /// Included relations are always preserved regardless of the `fields` param.
    pub fn apply_field_filter(&mut self, query_params: &QueryParams) {
        let fields = query_params.fields();
        let includes = query_params.includes();

        // Filter top-level fields
        if !fields.is_empty() {
            if !fields.contains(&"id".to_string()) {
                self.id = None;
            }
            if !fields.contains(&"name".to_string()) {
                self.name = None;
            }
            if !fields.contains(&"description".to_string()) {
                self.description = None;
            }
            if !fields.contains(&"client_id".to_string()) {
                self.client_id = None;
            }
            if !fields.contains(&"is_default".to_string()) {
                self.is_default = None;
            }
            if !fields.contains(&"permissions".to_string()) && !includes.contains(&"permissions".to_string()) {
                self.permissions = None;
            }
            if !fields.contains(&"client".to_string()) && !includes.contains(&"client".to_string()) {
                self.client = None;
            }
        }

        // Filter fields within included relations
        if let Some(selected_fields) = query_params.include_fields("permissions") {
            self.permissions = self.permissions.take().map(|perms| {
                perms
                    .into_iter()
                    .map(|p| p.filter_fields(selected_fields))
                    .collect()
            });
        }
        if let Some(selected_fields) = query_params.include_fields("client") {
            self.client = self.client.take().map(|c| c.filter_fields(selected_fields));
        }
    }
}

impl Into<RoleData> for ModelOptionDto {
    fn into(self) -> RoleData {
        let client_data: Option<ClientData> = self
            .client
            .and_then(|c| c.into_iter().next().map(|m| m.into()));

        let permissions_data: Option<Vec<PermissionData>> = self
            .permissions
            .map(|p| p.into_iter().map(|m| m.into()).collect());

        RoleData {
            name: self.name,
            description: self.description,
            id: self.id,
            client_id: self.client_id,
            is_default: self.is_default,
            permissions: permissions_data,
            client: client_data,
            ..Default::default()
        }
    }
}
