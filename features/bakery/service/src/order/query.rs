use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_bakery_entities::order::{ActiveModel, Column, Entity, ModelOptionDto};
use features_bakery_model::order::OrderData;

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
struct OrderQueryManager;



pub struct OrderQuery {}

impl OrderQuery {
    pub async fn get_by_id<'a>(id: i32) -> Result<OrderData, DbErr> {
        let model = OrderQueryManager::get_by_id_i32(id).await?;
        Ok(model.into())
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<OrderData>, DbErr> {
        let result = OrderQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
