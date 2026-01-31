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
        max = 16,
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

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct RoleData {
    id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
    client_id: Option<Uuid>,
    is_default: Option<bool>,
}

impl RoleData {
    pub fn get_id(&self) -> Option<Uuid> {
        self.id
    }
}

impl Into<RoleData> for ModelOptionDto {
    fn into(self) -> RoleData {
        RoleData {
            name: self.name,
            description: self.description,
            id: self.id,
            client_id: self.client_id,
            is_default: self.is_default,
            ..Default::default()
        }
    }
}
