use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_tagging_entities::entity_tag::{ActiveModel, Column, Entity, ModelOptionDto};
use features_tagging_entities::tag::Entity as TagEntity;
use features_tagging_model::entity_tag::EntityTagData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(TagEntity), field(tag), name("tag"))]
#[allow(dead_code)]
struct EntityTagQueryManager;

pub struct EntityTagQuery;

impl EntityTagQuery {
    pub async fn get_tags_for_entity(
        tenant_id: &str,
        entity_type: &str,
        entity_id: Uuid,
        pagination: &Pagination,
        _order: &Order,
        _query_params: &QueryParams,
    ) -> Result<QueryResult<EntityTagData>, AppError> {
        use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
        use shared_shared_config::db::DB_READ;

        let db = DB_READ
            .get()
            .ok_or(AppError::Internal("DB_READ not initialized".to_string()))?;

        let condition = Condition::all()
            .add(Column::TenantId.eq(tenant_id))
            .add(Column::EntityType.eq(entity_type))
            .add(Column::EntityId.eq(entity_id));

        let total = Entity::find()
            .filter(condition.clone())
            .count(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to count entity tags: {}", e)))?;

        let page_size = pagination.page_size.unwrap_or(20) as u64;
        let page = pagination.page.unwrap_or(1) as u64;
        let total_page = ((total as f64) / (page_size as f64)).ceil() as u64;

        let models = Entity::find()
            .filter(condition)
            .order_by_desc(Column::CreatedAt)
            .paginate(db.as_ref(), page_size)
            .fetch_page(page.saturating_sub(1))
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch entity tags: {}", e)))?;

        Ok(QueryResult {
            total_page: total_page,
            result: models.into_iter().map(|m| m.into()).collect(),
        })
    }

    pub async fn get_entities_for_tag(
        tenant_id: &str,
        tag_id: Uuid,
        pagination: &Pagination,
        _order: &Order,
    ) -> Result<QueryResult<EntityTagData>, AppError> {
        use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
        use shared_shared_config::db::DB_READ;

        let db = DB_READ
            .get()
            .ok_or(AppError::Internal("DB_READ not initialized".to_string()))?;

        let condition = Condition::all()
            .add(Column::TenantId.eq(tenant_id))
            .add(Column::TagId.eq(tag_id));

        let total = Entity::find()
            .filter(condition.clone())
            .count(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to count entity tags: {}", e)))?;

        let page_size = pagination.page_size.unwrap_or(20) as u64;
        let page = pagination.page.unwrap_or(1) as u64;
        let total_page = ((total as f64) / (page_size as f64)).ceil() as u64;

        let models = Entity::find()
            .filter(condition)
            .order_by_desc(Column::CreatedAt)
            .paginate(db.as_ref(), page_size)
            .fetch_page(page.saturating_sub(1))
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch entity tags: {}", e)))?;

        Ok(QueryResult {
            total_page: total_page,
            result: models.into_iter().map(|m| m.into()).collect(),
        })
    }

    /// Count how many entities a tag is associated with
    pub async fn get_usage_count(tag_id: Uuid) -> Result<i32, AppError> {
        use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
        use shared_shared_config::db::DB_READ;

        let db = DB_READ
            .get()
            .ok_or(AppError::Internal("DB_READ not initialized".to_string()))?;

        let count = Entity::find()
            .filter(Column::TagId.eq(tag_id))
            .count(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to count usage: {}", e)))?;

        Ok(count as i32)
    }
}
