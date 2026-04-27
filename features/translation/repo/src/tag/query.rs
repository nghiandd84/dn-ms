use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_translation_entities::tag::{ActiveModel, Column, Entity, ModelOptionDto};
use features_translation_model::TagData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TagQueryManager;

pub struct TagQuery;

impl TagQuery {
    pub async fn get_tag_by_id(tag_id: Uuid) -> Result<TagData, AppError> {
        let model = TagQueryManager::get_by_id_uuid(tag_id).await?;
        Ok(model.into())
    }

    pub async fn get_tags<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<TagData>, AppError> {
        let result = TagQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
