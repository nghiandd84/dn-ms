use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_tagging_entities::tag::{ActiveModel, Column, Entity, ModelOptionDto};
use features_tagging_entities::tag_group::Entity as TagGroupEntity;
use features_tagging_model::tag::TagData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(TagGroupEntity), field(tag_group), name("tag_group"))]
struct TagQueryManager;

pub struct TagQuery;

impl TagQuery {
    pub async fn get_tag_by_id(id: Uuid) -> Result<TagData, AppError> {
        let model = TagQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_tags(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
    ) -> Result<QueryResult<TagData>, AppError> {
        let includes = query_params.includes();
        let result = if !includes.is_empty() {
            TagQueryManager::filter_with_related_entities(
                pagination,
                order,
                filters,
                &includes,
                &vec![],
            )
            .await?
        } else {
            TagQueryManager::filter(pagination, order, filters).await?
        };
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    /// Fuzzy search tags by name for autocomplete
    pub async fn search_tags(
        tenant_id: &str,
        search_term: &str,
        limit: u64,
    ) -> Result<Vec<TagData>, AppError> {
        use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
        use shared_shared_config::db::DB_READ;

        let db = DB_READ
            .get()
            .ok_or(AppError::Internal("DB_READ not initialized".to_string()))?;

        let models = Entity::find()
            .filter(
                Condition::all()
                    .add(Column::TenantId.eq(tenant_id))
                    .add(Column::IsActive.eq(true))
                    .add(Column::AliasOf.is_null())
                    .add(Column::Name.contains(search_term.to_string())),
            )
            .order_by_asc(Column::Name)
            .paginate(db.as_ref(), limit)
            .fetch_page(0)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to search tags: {}", e)))?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }
}
