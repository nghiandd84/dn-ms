use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_translation_entities::tag::TagForCreateDto;
use features_translation_model::{TagData, TagForCreateRequest, TagForUpdateRequest};
use features_translation_repo::tag::{TagMutation, TagQuery};

pub struct TagService {}

impl TagService {
    pub async fn create_tag<'a>(tag_request: TagForCreateRequest) -> Result<Uuid, AppError> {
        let dto: TagForCreateDto = tag_request.into();
        let tag_id = TagMutation::create_tag(dto).await;
        let id = match tag_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating tag: {:?}", e);
                return Err(AppError::Internal("Failed to create tag".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_tag_by_id<'a>(tag_id: Uuid) -> Result<TagData, AppError> {
        TagQuery::get_tag_by_id(tag_id).await
    }

    pub async fn get_tags<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<TagData>, AppError> {
        TagQuery::get_tags(pagination, order, filters).await
    }

    pub async fn update_tag<'a>(
        tag_id: Uuid,
        tag_request: TagForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto: features_translation_entities::tag::TagForUpdateDto = tag_request.into();
        let result = TagMutation::update_tag(tag_id, dto).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating tag: {:?}", e);
                Err(AppError::Internal("Failed to update tag".to_string()))
            }
        }
    }

    pub async fn delete_tag<'a>(tag_id: Uuid) -> Result<bool, AppError> {
        let result = TagMutation::delete_tag(tag_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting tag: {:?}", e);
                Err(AppError::Internal("Failed to delete tag".to_string()))
            }
        }
    }
}
