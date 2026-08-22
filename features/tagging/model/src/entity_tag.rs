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

use features_tagging_entities::entity_tag::{EntityTagForCreateDto, Model, ModelOptionDto};

use super::tag::TagData;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct EntityTagData {
    pub id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub tenant_id: Option<String>,
    pub tagged_by: Option<Uuid>,
    pub created_at: Option<DateTime>,

    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub tag: Option<TagData>,
}

impl Into<EntityTagData> for Model {
    fn into(self) -> EntityTagData {
        EntityTagData {
            id: Some(self.id),
            tag_id: Some(self.tag_id),
            entity_type: Some(self.entity_type),
            entity_id: Some(self.entity_id),
            tenant_id: Some(self.tenant_id),
            tagged_by: Some(self.tagged_by),
            created_at: Some(self.created_at),
            tag: None,
        }
    }
}

impl Into<EntityTagData> for ModelOptionDto {
    fn into(self) -> EntityTagData {
        let tag_data: Option<TagData> = self
            .tag
            .and_then(|t| t.into_iter().next().map(|m| m.into()));

        EntityTagData {
            id: self.id,
            tag_id: self.tag_id,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            tenant_id: self.tenant_id,
            tagged_by: self.tagged_by,
            created_at: self.created_at,
            tag: tag_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct EntityTagForCreateRequest {
    pub tag_id: Uuid,
    #[validate(length(
        min = 1,
        max = 50,
        message = "entity_type must be between 1 and 50 characters"
    ))]
    pub entity_type: String,
    pub entity_id: Uuid,
}

impl EntityTagForCreateRequest {
    pub fn into_dto(self, tenant_id: String, tagged_by: Uuid) -> EntityTagForCreateDto {
        EntityTagForCreateDto {
            tag_id: self.tag_id,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            tenant_id,
            tagged_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BulkTagRequest {
    pub tag_ids: Vec<Uuid>,
    #[validate(length(
        min = 1,
        max = 50,
        message = "entity_type must be between 1 and 50 characters"
    ))]
    pub entity_type: String,
    pub entity_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BulkUntagRequest {
    pub tag_ids: Vec<Uuid>,
    #[validate(length(
        min = 1,
        max = 50,
        message = "entity_type must be between 1 and 50 characters"
    ))]
    pub entity_type: String,
    pub entity_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkTagResponse {
    pub ok: bool,
    pub created: i32,
    pub skipped: i32,
}
