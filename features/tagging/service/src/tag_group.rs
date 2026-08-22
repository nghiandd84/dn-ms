use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_tagging_model::tag_group::{
    TagGroupData, TagGroupForCreateRequest, TagGroupForUpdateRequest,
};
use features_tagging_repo::{TagGroupMutation, TagGroupQuery};

pub struct TagGroupService;

impl TagGroupService {
    pub async fn create_tag_group(
        tenant_id: String,
        req: TagGroupForCreateRequest,
    ) -> Result<Uuid, AppError> {
        // Validate hierarchy depth: if parent_id is set, ensure it has no parent itself (max 2 levels)
        if let Some(parent_id) = req.parent_id {
            let parent = TagGroupQuery::get_tag_group_by_id(parent_id).await?;
            if parent.parent_id.is_some() {
                return Err(AppError::Internal(
                    "Cannot create sub-group under a sub-group. Maximum hierarchy depth is 2 levels.".to_string(),
                ));
            }
        }

        let dto = req.into_dto(tenant_id);
        let id = TagGroupMutation::create_tag_group(dto).await.map_err(|e| {
            debug!("Error creating tag group: {:?}", e);
            AppError::Internal("Failed to create tag group".to_string())
        })?;
        Ok(id)
    }

    pub async fn get_tag_groups(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
        query_params: &QueryParams,
    ) -> Result<QueryResult<TagGroupData>, AppError> {
        TagGroupQuery::get_tag_groups(pagination, order, filters, query_params).await
    }

    pub async fn get_tag_group_by_id(id: Uuid) -> Result<TagGroupData, AppError> {
        TagGroupQuery::get_tag_group_by_id(id).await
    }

    pub async fn update_tag_group(
        id: Uuid,
        req: TagGroupForUpdateRequest,
    ) -> Result<bool, AppError> {
        // If updating parent_id, validate hierarchy
        if let Some(parent_id) = req.parent_id {
            // Cannot be its own parent
            if parent_id == id {
                return Err(AppError::Internal(
                    "A tag group cannot be its own parent".to_string(),
                ));
            }
            let parent = TagGroupQuery::get_tag_group_by_id(parent_id).await?;
            if parent.parent_id.is_some() {
                return Err(AppError::Internal(
                    "Cannot move under a sub-group. Maximum hierarchy depth is 2 levels."
                        .to_string(),
                ));
            }
        }

        let result = TagGroupMutation::update_tag_group(id, req.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating tag group: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update tag group".to_string(),
                ))
            }
        }
    }

    pub async fn delete_tag_group(id: Uuid) -> Result<bool, AppError> {
        let result = TagGroupMutation::delete_tag_group(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting tag group: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete tag group".to_string(),
                ))
            }
        }
    }
}
