use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_error::app::AppError;

use features_lookup_model::lookup_item_translation::{
    LookupItemTranslationData, LookupItemTranslationForCreateRequest,
    LookupItemTranslationForUpdateRequest,
};
use features_lookup_repo::{LookupItemTranslationMutation, LookupItemTranslationQuery};

pub struct LookupItemTranslationService {}

impl LookupItemTranslationService {
    pub async fn create_translation(
        req: LookupItemTranslationForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let lookup_item_translation_id =
            LookupItemTranslationMutation::create_translation(req.into()).await;
        let id = match lookup_item_translation_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating lookup_item_translation: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create lookup_item_translation".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_translation_by_id(id: Uuid) -> Result<LookupItemTranslationData, AppError> {
        LookupItemTranslationQuery::get_translation_by_id(id).await
    }

    pub async fn search_translations_by_item_id(
        item_id: Uuid,
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<LookupItemTranslationData>, AppError> {
        LookupItemTranslationQuery::get_translations_by_item_id(item_id, pagination, order, filters)
            .await
    }

    pub async fn get_translations(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<LookupItemTranslationData>, AppError> {
        LookupItemTranslationQuery::get_translations(pagination, order, filters).await
    }

    pub async fn update_translation(
        lookup_translation_item_id: Uuid,
        req: LookupItemTranslationForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = LookupItemTranslationMutation::update_translation(
            lookup_translation_item_id,
            req.into(),
        )
        .await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating lookup_item_translation: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update lookup_item_translation".to_string(),
                ))
            }
        }
    }

    pub async fn delete_translation(lookup_translation_item_id: Uuid) -> Result<bool, AppError> {
        let result =
            LookupItemTranslationMutation::delete_translation(lookup_translation_item_id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting lookup_item_translation: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete lookup_item_translation".to_string(),
                ))
            }
        }
    }
}
