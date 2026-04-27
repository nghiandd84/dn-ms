use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::scope::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::scope::ScopeData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ScopeQueryManager;

pub struct ScopeQuery {}

impl ScopeQuery {
    pub async fn get<'a>(id: Uuid) -> Result<ScopeData, DbErr> {
        let model = ScopeQueryManager::get_by_id_uuid(id).await?;
        let user_data: ScopeData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ScopeData>, DbErr> {
        let result = ScopeQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
