use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::scope::{ModelOptionDto, ScopeForCreateDto, ScopeForUpdateDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ScopeForCreateRequest {
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
}

impl Into<ScopeForCreateDto> for ScopeForCreateRequest {
    fn into(self) -> ScopeForCreateDto {
        ScopeForCreateDto {
            name: self.name,
            description: self.description.unwrap_or_default(),
        }
    }
}


#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ScopeForUpdateRequest {
    #[validate(length(
        min = 0,
        max = 512,
        code = "description",
        message = "the length of description must be between 0 and 512"
    ))]
    pub description: Option<String>,
}

impl Into<ScopeForUpdateDto> for ScopeForUpdateRequest {
    fn into(self) -> ScopeForUpdateDto {
        ScopeForUpdateDto {
            description: self.description,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct ScopeData {
    id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
}

impl Into<ScopeData> for ModelOptionDto {
    fn into(self) -> ScopeData {
        ScopeData {
            name: self.name,
            description: self.description,
            id: self.id,
            ..Default::default()
        }
    }
}
