use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_translation_entities::translation_version::{
    TranslationVersionForCreateDto, TranslationVersionForUpdateDto,
};
use features_translation_model::{
    TranslationVersionData, TranslationVersionForCreateRequest, TranslationVersionForUpdateRequest,
};
use features_translation_repo::translation_version::{
    TranslationVersionMutation, TranslationVersionQuery,
};

pub struct TranslationVersionService {}

impl TranslationVersionService {
    pub async fn create_translation_version<'a>(
        translation_version_request: TranslationVersionForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: TranslationVersionForCreateDto = translation_version_request.into();
        let version_id = TranslationVersionMutation::create_translation_version(dto).await;
        let id = match version_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating translation version: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create translation version".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_translation_version_by_id<'a>(
        db: &'a DbConn,
        version_id: Uuid,
    ) -> Result<TranslationVersionData, AppError> {
        TranslationVersionQuery::get_translation_version_by_id(db, version_id).await
    }

    pub async fn get_translation_versions<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        TranslationVersionQuery::get_translation_versions(db, pagination, order, filters).await
    }

    pub async fn get_latest_version_by_key_locale<'a>(
        db: &'a DbConn,
        key_id: Uuid,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        TranslationVersionQuery::get_latest_version_by_key_locale(
            db, key_id, filters, pagination, order,
        )
        .await
    }

    pub async fn update_translation_version<'a>(
        version_id: Uuid,
        translation_version_request: TranslationVersionForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto: TranslationVersionForUpdateDto = translation_version_request.into();
        let result = TranslationVersionMutation::update_translation_version(version_id, dto).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating translation version: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update translation version".to_string(),
                ))
            }
        }
    }

    pub async fn delete_translation_version<'a>(
        version_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = TranslationVersionMutation::delete_translation_version(version_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting translation version: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete translation version".to_string(),
                ))
            }
        }
    }
}
