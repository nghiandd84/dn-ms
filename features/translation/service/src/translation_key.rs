use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_translation_entities::translation_key::TranslationKeyForCreateDto;
use features_translation_model::{TranslationKeyData, TranslationKeyForCreateRequest, TranslationKeyForUpdateRequest};
use features_translation_repo::translation_key::{TranslationKeyMutation, TranslationKeyQuery};

pub struct TranslationKeyService {}

impl TranslationKeyService {
    pub async fn create_translation_key<'a>(
        db: &'a DbConn,
        translation_key_request: TranslationKeyForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: TranslationKeyForCreateDto = translation_key_request.into();
        let key_id = TranslationKeyMutation::create_translation_key(db, dto).await;
        let id = match key_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating translation key: {:?}", e);
                return Err(AppError::Internal("Failed to create translation key".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_translation_key_by_id<'a>(
        db: &'a DbConn,
        key_id: Uuid,
    ) -> Result<TranslationKeyData, AppError> {
        TranslationKeyQuery::get_translation_key_by_id(db, key_id).await
    }

    pub async fn get_translation_keys_by_project<'a>(
        db: &'a DbConn,
        project_id: Uuid,
    ) -> Result<QueryResult<TranslationKeyData>, AppError> {
        TranslationKeyQuery::get_translation_keys_by_project(db, project_id).await
    }

    pub async fn get_translation_keys<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TranslationKeyData>, AppError> {
        TranslationKeyQuery::get_translation_keys(db, pagination, order, filters).await
    }

    pub async fn update_translation_key<'a>(
        db: &'a DbConn,
        key_id: Uuid,
        translation_key_request: TranslationKeyForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto: features_translation_entities::translation_key::TranslationKeyForUpdateDto = translation_key_request.into();
        let result = TranslationKeyMutation::update_translation_key(db, key_id, dto).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating translation key: {:?}", e);
                Err(AppError::Internal("Failed to update translation key".to_string()))
            }
        }
    }

    pub async fn delete_translation_key<'a>(
        db: &'a DbConn,
        key_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = TranslationKeyMutation::delete_translation_key(db, key_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting translation key: {:?}", e);
                Err(AppError::Internal("Failed to delete translation key".to_string()))
            }
        }
    }
}
