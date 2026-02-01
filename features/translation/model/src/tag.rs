use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_translation_entities::tag::{ModelOptionDto, TagForCreateDto, TagForUpdateDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TagForCreateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "name",
        message = "the length of name must be between 1 and 50"
    ))]
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct TagForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "name",
        message = "the length of name must be between 1 and 50"
    ))]
    pub name: Option<String>,
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TagData {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}



impl Into<TagData> for ModelOptionDto {
    fn into(self) -> TagData {
        TagData {
            id: self.id,
            name: self.name,
        }
    }
}

impl From<TagForCreateRequest> for TagForCreateDto {
    fn from(req: TagForCreateRequest) -> Self {
        TagForCreateDto { name: req.name }
    }
}

impl From<TagForUpdateRequest> for TagForUpdateDto {
    fn from(req: TagForUpdateRequest) -> Self {
        TagForUpdateDto { name: req.name }
    }
}
