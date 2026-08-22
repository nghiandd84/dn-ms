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

use features_tagging_entities::tag::{Model, ModelOptionDto, TagForCreateDto, TagForUpdateDto};

use super::tag_group::TagGroupData;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct TagData {
    pub id: Option<Uuid>,
    pub tenant_id: Option<String>,
    pub tag_group_id: Option<Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub alias_of: Option<Uuid>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
    pub usage_count: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,

    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub tag_group: Option<TagGroupData>,
}

impl Into<TagData> for Model {
    fn into(self) -> TagData {
        TagData {
            id: Some(self.id),
            tenant_id: Some(self.tenant_id),
            tag_group_id: Some(self.tag_group_id),
            name: Some(self.name),
            slug: Some(self.slug),
            color: Some(self.color),
            description: Some(self.description),
            alias_of: self.alias_of,
            is_active: Some(self.is_active),
            sort_order: Some(self.sort_order),
            usage_count: Some(self.usage_count),
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at),
            tag_group: None,
        }
    }
}

impl Into<TagData> for ModelOptionDto {
    fn into(self) -> TagData {
        let tag_group_data: Option<TagGroupData> = self
            .tag_group
            .and_then(|tg| tg.into_iter().next().map(|m| m.into()));

        TagData {
            id: self.id,
            tenant_id: self.tenant_id,
            tag_group_id: self.tag_group_id,
            name: self.name,
            slug: self.slug,
            color: self.color,
            description: self.description,
            alias_of: self.alias_of.flatten(),
            is_active: self.is_active,
            sort_order: self.sort_order,
            usage_count: self.usage_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
            tag_group: tag_group_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct TagForCreateRequest {
    pub tag_group_id: Uuid,
    #[validate(length(
        min = 1,
        max = 100,
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 120,
        message = "slug must be between 1 and 120 characters"
    ))]
    pub slug: String,
    #[validate(length(max = 7, message = "color must be a valid hex color (e.g. #FF0000)"))]
    pub color: Option<String>,
    #[validate(length(max = 500, message = "description must not exceed 500 characters"))]
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

impl TagForCreateRequest {
    pub fn into_dto(self, tenant_id: String) -> TagForCreateDto {
        TagForCreateDto {
            tenant_id,
            tag_group_id: self.tag_group_id,
            name: self.name,
            slug: self.slug,
            color: self.color.unwrap_or_else(|| "#000000".to_string()),
            description: self.description.unwrap_or_default(),
            alias_of: None,
            is_active: true,
            sort_order: self.sort_order.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct TagForUpdateRequest {
    pub tag_group_id: Option<Uuid>,
    #[validate(length(
        min = 1,
        max = 100,
        message = "name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(
        min = 1,
        max = 120,
        message = "slug must be between 1 and 120 characters"
    ))]
    pub slug: Option<String>,
    #[validate(length(max = 7, message = "color must be a valid hex color (e.g. #FF0000)"))]
    pub color: Option<String>,
    #[validate(length(max = 500, message = "description must not exceed 500 characters"))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

impl Into<TagForUpdateDto> for TagForUpdateRequest {
    fn into(self) -> TagForUpdateDto {
        TagForUpdateDto {
            tag_group_id: self.tag_group_id,
            name: self.name,
            slug: self.slug,
            color: self.color,
            description: self.description,
            alias_of: None,
            is_active: self.is_active,
            sort_order: self.sort_order,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct TagMergeRequest {
    /// The tag ID to merge into (canonical tag)
    pub target_tag_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagUsageResponse {
    pub tag_id: Uuid,
    pub usage_count: i32,
}
