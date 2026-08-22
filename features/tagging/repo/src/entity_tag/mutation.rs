use shared_shared_data_error::app::AppError;
use shared_shared_macro::Mutation;

use features_tagging_entities::entity_tag::{
    ActiveModel, Column, Entity, EntityTagForCreateDto, Model, ModelOptionDto,
};

use crate::entity_tag::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct EntityTagMutationManager {}

pub struct EntityTagMutation;

impl EntityTagMutation {
    pub fn create_entity_tag<'a>(
        data: EntityTagForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        EntityTagMutationManager::create_uuid(data.into())
    }

    pub fn delete_entity_tag<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EntityTagMutationManager::delete_by_id_uuid(id)
    }

    /// Delete entity tag by (tag_id, entity_type, entity_id) combination
    pub async fn delete_by_tag_and_entity(
        tag_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<bool, AppError> {
        use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let result = Entity::delete_many()
            .filter(
                Condition::all()
                    .add(Column::TagId.eq(tag_id))
                    .add(Column::EntityType.eq(entity_type))
                    .add(Column::EntityId.eq(entity_id)),
            )
            .exec(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete entity tag: {}", e)))?;

        Ok(result.rows_affected > 0)
    }

    /// Bulk create entity tags (skip duplicates)
    pub async fn bulk_create(items: Vec<EntityTagForCreateDto>) -> Result<(i32, i32), AppError> {
        use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let mut created = 0;
        let mut skipped = 0;

        for item in items {
            // Check if already exists
            let existing = Entity::find()
                .filter(
                    Condition::all()
                        .add(Column::TagId.eq(item.tag_id))
                        .add(Column::EntityType.eq(&item.entity_type))
                        .add(Column::EntityId.eq(item.entity_id)),
                )
                .one(db.as_ref())
                .await
                .map_err(|e| AppError::Internal(format!("Failed to check existing: {}", e)))?;

            if existing.is_some() {
                skipped += 1;
                continue;
            }

            let active = ActiveModel {
                id: ActiveValue::Set(uuid::Uuid::new_v4()),
                tag_id: ActiveValue::Set(item.tag_id),
                entity_type: ActiveValue::Set(item.entity_type),
                entity_id: ActiveValue::Set(item.entity_id),
                tenant_id: ActiveValue::Set(item.tenant_id),
                tagged_by: ActiveValue::Set(item.tagged_by),
                created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            };

            active
                .insert(db.as_ref())
                .await
                .map_err(|e| AppError::Internal(format!("Failed to create entity tag: {}", e)))?;
            created += 1;
        }

        Ok((created, skipped))
    }

    /// Bulk delete entity tags
    pub async fn bulk_delete(
        tag_ids: &[Uuid],
        entity_type: &str,
        entity_ids: &[Uuid],
    ) -> Result<u64, AppError> {
        use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        let result = Entity::delete_many()
            .filter(
                Condition::all()
                    .add(Column::TagId.is_in(tag_ids.to_vec()))
                    .add(Column::EntityType.eq(entity_type))
                    .add(Column::EntityId.is_in(entity_ids.to_vec())),
            )
            .exec(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to bulk delete: {}", e)))?;

        Ok(result.rows_affected)
    }

    /// Reassign all entity_tags from one tag to another (used during merge)
    pub async fn reassign_tag(source_tag_id: Uuid, target_tag_id: Uuid) -> Result<u64, AppError> {
        use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter};
        use shared_shared_config::db::DB_WRITE;

        let db = DB_WRITE
            .get()
            .ok_or(AppError::Internal("DB_WRITE not initialized".to_string()))?;

        // Find all entity_tags for the source tag
        let entity_tags = Entity::find()
            .filter(Column::TagId.eq(source_tag_id))
            .all(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to find entity tags: {}", e)))?;

        let mut reassigned = 0u64;

        for entity_tag in entity_tags {
            // Check if target already has this association
            let existing = Entity::find()
                .filter(
                    Condition::all()
                        .add(Column::TagId.eq(target_tag_id))
                        .add(Column::EntityType.eq(&entity_tag.entity_type))
                        .add(Column::EntityId.eq(entity_tag.entity_id)),
                )
                .one(db.as_ref())
                .await
                .map_err(|e| AppError::Internal(format!("Failed to check existing: {}", e)))?;

            if existing.is_some() {
                // Delete the source association (would be a duplicate)
                let active: ActiveModel = entity_tag.into();
                active
                    .delete(db.as_ref())
                    .await
                    .map_err(|e| AppError::Internal(format!("Failed to delete: {}", e)))?;
            } else {
                // Reassign to target
                let mut active: ActiveModel = entity_tag.into();
                active.tag_id = ActiveValue::Set(target_tag_id);
                active
                    .update(db.as_ref())
                    .await
                    .map_err(|e| AppError::Internal(format!("Failed to reassign: {}", e)))?;
                reassigned += 1;
            }
        }

        Ok(reassigned)
    }
}
