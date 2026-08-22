use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_tagging_entities::tag_group::{
    Model, ModelOptionDto, TagGroupForCreateDto, TagGroupForUpdateDto,
};

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct TagGroupData {
    pub id: Option<Uuid>,
    pub tenant_id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,

    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<Object>>)]
    pub children: Option<Vec<TagGroupData>>,
}

impl Into<TagGroupData> for Model {
    fn into(self) -> TagGroupData {
        TagGroupData {
            id: Some(self.id),
            tenant_id: Some(self.tenant_id),
            code: Some(self.code),
            name: Some(self.name),
            description: Some(self.description),
            parent_id: self.parent_id,
            is_active: Some(self.is_active),
            sort_order: Some(self.sort_order),
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at),
            children: None,
        }
    }
}

impl Into<TagGroupData> for ModelOptionDto {
    fn into(self) -> TagGroupData {
        TagGroupData {
            id: self.id,
            tenant_id: self.tenant_id,
            code: self.code,
            name: self.name,
            description: self.description,
            parent_id: self.parent_id.flatten(),
            is_active: self.is_active,
            sort_order: self.sort_order,
            created_at: self.created_at,
            updated_at: self.updated_at,
            children: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct TagGroupForCreateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(max = 500, message = "description must not exceed 500 characters"))]
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<i32>,
}

impl TagGroupForCreateRequest {
    pub fn into_dto(self, tenant_id: String) -> TagGroupForCreateDto {
        TagGroupForCreateDto {
            tenant_id,
            code: self.code,
            name: self.name,
            description: self.description.unwrap_or_default(),
            parent_id: self.parent_id,
            is_active: true,
            sort_order: self.sort_order.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct TagGroupForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: Option<String>,
    #[validate(length(
        min = 1,
        max = 100,
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(max = 500, message = "description must not exceed 500 characters"))]
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

impl Into<TagGroupForUpdateDto> for TagGroupForUpdateRequest {
    fn into(self) -> TagGroupForUpdateDto {
        TagGroupForUpdateDto {
            code: self.code,
            name: self.name,
            description: self.description,
            parent_id: Some(self.parent_id),
            is_active: self.is_active,
            sort_order: self.sort_order,
        }
    }
}
