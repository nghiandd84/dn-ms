use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::role_permission::{ModelOptionDto, RolePermissionForCreateDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RolePermissionForCreateRequest {
    #[validate(required(code = "role_id_required", message = "role_id is required"))]
    pub role_id: Option<Uuid>,
    #[validate(required(code = "permission_id_required", message = "permission_id is required"))]
    pub permission_id: Option<Uuid>,
}

impl Into<RolePermissionForCreateDto> for RolePermissionForCreateRequest {
    fn into(self) -> RolePermissionForCreateDto {
        RolePermissionForCreateDto {
            role_id: self.role_id.unwrap(),
            permission_id: self.permission_id.unwrap(),
        }
    }
}
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct RolePermissionData {
    pub role_id: Option<Uuid>,
    pub permission_id: Option<Uuid>,
}

impl Into<RolePermissionData> for ModelOptionDto {
    fn into(self) -> RolePermissionData {
        RolePermissionData {
            role_id: self.role_id,
            permission_id: self.permission_id,
            ..Default::default()
        }
    }
}
