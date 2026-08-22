use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_tagging_model::tag::{TagData, TagForCreateRequest, TagForUpdateRequest, TagUsageResponse};
use features_tagging_repo::{EntityTagMutation, EntityTagQuery, TagMutation, TagQuery};

pub struct TagService;

impl TagService {
    pub async fn create_tag(tenant_id: String, req: TagForCreateRequest) -> Result<Uuid, AppError> {
        let dto = req.into_dto(tenant_id);
        let id = TagMutation::create_tag(dto).await.map_err(|e| {
            debug!("Error creating tag: {:?}", e);
            AppError::Internal("Failed to create tag".to_string())
        })?;
        Ok(id)
    }

    pub async fn get_tags(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
        query_params: &QueryParams,
    ) -> Result<QueryResult<TagData>, AppError> {
        TagQuery::get_tags(pagination, order, filters, query_params).await
    }

    pub async fn get_tag_by_id(id: Uuid) -> Result<TagData, AppError> {
        TagQuery::get_tag_by_id(id).await
    }

    pub async fn update_tag(id: Uuid, req: TagForUpdateRequest) -> Result<bool, AppError> {
        let result = TagMutation::update_tag(id, req.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating tag: {:?}", e);
                Err(AppError::Internal("Failed to update tag".to_string()))
            }
        }
    }

    pub async fn delete_tag(id: Uuid) -> Result<bool, AppError> {
        let result = TagMutation::delete_tag(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting tag: {:?}", e);
                Err(AppError::Internal("Failed to delete tag".to_string()))
            }
        }
    }

    /// Fuzzy search tags for autocomplete
    pub async fn search_tags(
        tenant_id: &str,
        search_term: &str,
        limit: Option<u64>,
    ) -> Result<Vec<TagData>, AppError> {
        let limit = limit.unwrap_or(10);
        TagQuery::search_tags(tenant_id, search_term, limit).await
    }

    /// Get usage count for a tag
    pub async fn get_usage_count(tag_id: Uuid) -> Result<TagUsageResponse, AppError> {
        let count = EntityTagQuery::get_usage_count(tag_id).await?;
        Ok(TagUsageResponse {
            tag_id,
            usage_count: count,
        })
    }

    /// Merge source tag into target tag:
    /// 1. Reassign all entity_tags from source to target
    /// 2. Mark source as alias of target
    /// 3. Update usage counts
    pub async fn merge_tags(source_tag_id: Uuid, target_tag_id: Uuid) -> Result<(), AppError> {
        if source_tag_id == target_tag_id {
            return Err(AppError::Internal(
                "Cannot merge a tag into itself".to_string(),
            ));
        }

        // Verify both tags exist
        let _source = TagQuery::get_tag_by_id(source_tag_id).await?;
        let _target = TagQuery::get_tag_by_id(target_tag_id).await?;

        // Reassign entity_tags from source to target
        let reassigned = EntityTagMutation::reassign_tag(source_tag_id, target_tag_id).await?;
        debug!(
            "Merged tag {}: reassigned {} entity_tags to {}",
            source_tag_id, reassigned, target_tag_id
        );

        // Mark source as alias of target
        TagMutation::set_alias(source_tag_id, target_tag_id).await?;

        Ok(())
    }
}
