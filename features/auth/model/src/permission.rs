use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::permission::{ModelOptionDto, PermissionForCreateDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct PermissionForCreateRequest {
    #[validate(length(
        min = 10,
        max = 1024,
        code = "resource",
        message = "the length of resource must be between 10 and 1024"
    ))]
    pub resource: String,
    pub description: Option<String>,
    pub mask: Option<i32>,
}

impl Into<PermissionForCreateDto> for PermissionForCreateRequest {
    fn into(self) -> PermissionForCreateDto {
        PermissionForCreateDto {
            resource: self.resource,
            description: self.description,
            mask: self.mask.unwrap_or(0), // Default mask value
        }
    }
}
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PermissionData {
    pub resource: Option<String>,
    pub description: Option<String>,
    pub mask: Option<i32>,
    pub id: Option<Uuid>,
}

impl Into<PermissionData> for ModelOptionDto {
    fn into(self) -> PermissionData {
        PermissionData {
            resource: self.resource,
            description: self.description.unwrap(),
            id: self.id,
            mask: self.mask,
            ..Default::default()
        }
    }
}
