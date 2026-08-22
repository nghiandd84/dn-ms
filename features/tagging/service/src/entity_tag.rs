use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_tagging_model::entity_tag::{
    BulkTagRequest, BulkTagResponse, BulkUntagRequest, EntityTagData, EntityTagForCreateRequest,
};
use features_tagging_repo::{EntityTagMutation, EntityTagQuery, TagMutation, TagQuery};

use features_tagging_entities::entity_tag::EntityTagForCreateDto;

pub struct EntityTagService;

impl EntityTagService {
    /// Assign a single tag to an entity
    pub async fn assign_tag(
        tenant_id: String,
        tagged_by: Uuid,
        req: EntityTagForCreateRequest,
    ) -> Result<Uuid, AppError> {
        // Resolve alias: if the tag is an alias, use the canonical tag
        let tag = TagQuery::get_tag_by_id(req.tag_id).await?;
        let actual_tag_id = if let Some(alias_of) = tag.alias_of {
            alias_of
        } else {
            req.tag_id
        };

        let dto = EntityTagForCreateDto {
            tag_id: actual_tag_id,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            tenant_id,
            tagged_by,
        };

        let id = EntityTagMutation::create_entity_tag(dto)
            .await
            .map_err(|e| {
                debug!("Error creating entity tag: {:?}", e);
                AppError::Internal("Failed to assign tag".to_string())
            })?;

        // Increment usage count
        let _ = TagMutation::increment_usage_count(actual_tag_id).await;

        Ok(id)
    }

    /// Remove a tag from an entity
    pub async fn remove_tag(
        tag_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<bool, AppError> {
        let deleted =
            EntityTagMutation::delete_by_tag_and_entity(tag_id, entity_type, entity_id).await?;

        if deleted {
            let _ = TagMutation::decrement_usage_count(tag_id).await;
        }

        Ok(deleted)
    }

    /// Bulk assign tags to entities
    pub async fn bulk_assign(
        tenant_id: String,
        tagged_by: Uuid,
        req: BulkTagRequest,
    ) -> Result<BulkTagResponse, AppError> {
        let mut items = Vec::new();

        for tag_id in &req.tag_ids {
            // Resolve alias
            let tag = TagQuery::get_tag_by_id(*tag_id).await?;
            let actual_tag_id = tag.alias_of.unwrap_or(*tag_id);

            for entity_id in &req.entity_ids {
                items.push(EntityTagForCreateDto {
                    tag_id: actual_tag_id,
                    entity_type: req.entity_type.clone(),
                    entity_id: *entity_id,
                    tenant_id: tenant_id.clone(),
                    tagged_by,
                });
            }
        }

        let (created, skipped) = EntityTagMutation::bulk_create(items).await?;

        Ok(BulkTagResponse {
            ok: true,
            created,
            skipped,
        })
    }

    /// Bulk remove tags from entities
    pub async fn bulk_remove(req: BulkUntagRequest) -> Result<BulkTagResponse, AppError> {
        let removed =
            EntityTagMutation::bulk_delete(&req.tag_ids, &req.entity_type, &req.entity_ids)
                .await?;

        Ok(BulkTagResponse {
            ok: true,
            created: 0,
            skipped: removed as i32,
        })
    }

    /// Get all tags for a specific entity
    pub async fn get_tags_for_entity(
        tenant_id: &str,
        entity_type: &str,
        entity_id: Uuid,
        pagination: &Pagination,
        order: &Order,
        query_params: &QueryParams,
    ) -> Result<QueryResult<EntityTagData>, AppError> {
        EntityTagQuery::get_tags_for_entity(
            tenant_id,
            entity_type,
            entity_id,
            pagination,
            order,
            query_params,
        )
        .await
    }

    /// Get all entities for a specific tag
    pub async fn get_entities_for_tag(
        tenant_id: &str,
        tag_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<EntityTagData>, AppError> {
        EntityTagQuery::get_entities_for_tag(tenant_id, tag_id, pagination, order).await
    }
}
