use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::role::{ModelOptionDto, RoleForCreateDto};
use shared_shared_macro::{ParamFilter, Response};
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RoleForCreateRequest {
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
}

impl Into<RoleForCreateDto> for RoleForCreateRequest {
    fn into(self) -> RoleForCreateDto {
        RoleForCreateDto {
            name: self.name,
            description: self.description,
        }
    }
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
}

impl Into<RoleData> for ModelOptionDto {
    fn into(self) -> RoleData {
        RoleData {
            name: self.name,
            description: self.description,
            id: self.id,
            ..Default::default()
        }
    }
}
