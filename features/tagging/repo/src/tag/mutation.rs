use shared_shared_data_error::app::AppError;
use shared_shared_macro::Mutation;

use features_tagging_entities::tag::{
    ActiveModel, Column, Entity, Model, TagForCreateDto, TagForUpdateDto, ModelOptionDto,
};

use crate::tag::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TagMutationManager {}

pub struct TagMutation;

impl TagMutation {
    pub fn create_tag<'a>(
        data: TagForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TagMutationManager::create_uuid(data.into())
    }

    pub fn update_tag<'a>(
        id: Uuid,
        data: TagForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete_tag<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagMutationManager::delete_by_id_uuid(id)
    }

    /// Increment usage_count by 1
    pub async fn increment_usage_count(tag_id: Uuid) -> Result<(), AppError> {
        use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let tag = Entity::find_by_id(tag_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to find tag: {}", e)))?
            .ok_or(AppError::EntityNotFound { entity: "Tag".to_string() })?;

        let mut active: ActiveModel = tag.into();
        let current_count = match &active.usage_count {
            ActiveValue::Unchanged(v) => *v,
            _ => 0,
        };
        active.usage_count = ActiveValue::Set(current_count + 1);
        active
            .update(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to increment usage count: {}", e)))?;
        Ok(())
    }

    /// Decrement usage_count by 1 (min 0)
    pub async fn decrement_usage_count(tag_id: Uuid) -> Result<(), AppError> {
        use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let tag = Entity::find_by_id(tag_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to find tag: {}", e)))?
            .ok_or(AppError::EntityNotFound { entity: "Tag".to_string() })?;

        let mut active: ActiveModel = tag.into();
        let current_count = match &active.usage_count {
            ActiveValue::Unchanged(v) => *v,
            _ => 0,
        };
        active.usage_count = ActiveValue::Set((current_count - 1).max(0));
        active
            .update(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to decrement usage count: {}", e)))?;
        Ok(())
    }

    /// Set alias_of on a source tag (used during merge)
    pub async fn set_alias(source_tag_id: Uuid, target_tag_id: Uuid) -> Result<(), AppError> {
        use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let tag = Entity::find_by_id(source_tag_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to find tag: {}", e)))?
            .ok_or(AppError::EntityNotFound { entity: "Source tag".to_string() })?;

        let mut active: ActiveModel = tag.into();
        active.alias_of = ActiveValue::Set(Some(target_tag_id));
        active.is_active = ActiveValue::Set(false);
        active
            .update(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to set alias: {}", e)))?;
        Ok(())
    }
}
