use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_tagging_entities::tag_group::{ActiveModel, Column, Entity, ModelOptionDto};
use features_tagging_model::tag_group::TagGroupData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TagGroupQueryManager;

pub struct TagGroupQuery;

impl TagGroupQuery {
    pub async fn get_tag_group_by_id(id: Uuid) -> Result<TagGroupData, AppError> {
        let model = TagGroupQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_tag_groups(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        _query_params: &QueryParams,
    ) -> Result<QueryResult<TagGroupData>, AppError> {
        let result = TagGroupQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
