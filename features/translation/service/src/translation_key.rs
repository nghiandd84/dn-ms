use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_translation_entities::{
    key_tag::KeyTagForCreateDto, translation_key::TranslationKeyForCreateDto,
};
use features_translation_model::{
    AssignTagsRequest, TranslationKeyData, TranslationKeyForCreateRequest,
    TranslationKeyForUpdateRequest, UnassignTagsRequest,
};
use features_translation_repo::{
    key_tag::{KeyTagMutation, KeyTagQueryManager},
    translation_key::{TranslationKeyMutation, TranslationKeyQuery},
};

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
                return Err(AppError::Internal(
                    "Failed to create translation key".to_string(),
                ));
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
        let dto: features_translation_entities::translation_key::TranslationKeyForUpdateDto =
            translation_key_request.into();
        let result = TranslationKeyMutation::update_translation_key(db, key_id, dto).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating translation key: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update translation key".to_string(),
                ))
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
                Err(AppError::Internal(
                    "Failed to delete translation key".to_string(),
                ))
            }
        }
    }

    pub async fn assign_tags_to_key<'a>(
        db: &'a DbConn,
        key_id: Uuid,
        req: AssignTagsRequest,
    ) -> Result<bool, AppError> {
        let mut success = false;
        for tag_id in req.tag_ids {
            let exists = KeyTagQueryManager::key_tag_exists(db, key_id, tag_id).await?;
            if !exists {
                let dto = KeyTagForCreateDto { key_id, tag_id };
                match KeyTagMutation::create_key_tag(db, dto).await {
                    Ok(_) => success = true,
                    Err(e) => {
                        debug!("Error assigning tag to key: {:?}", e);
                        return Err(AppError::Internal(
                            "Failed to assign tag to key".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(success)
    }

    pub async fn unassign_tags_from_key<'a>(
        db: &'a DbConn,
        key_id: Uuid,
        req: UnassignTagsRequest,
    ) -> Result<bool, AppError> {
        let mut success = false;
        for tag_id in req.tag_ids {
            match KeyTagMutation::delete_key_tag(db, key_id, tag_id).await {
                Ok(deleted) => {
                    if deleted {
                        success = true;
                    }
                }
                Err(e) => {
                    debug!("Error unassigning tag from key: {:?}", e);
                    return Err(AppError::Internal(
                        "Failed to unassign tag from key".to_string(),
                    ));
                }
            }
        }
        Ok(success)
    }
}
