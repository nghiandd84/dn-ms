use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_bakery_entities::customer::{ActiveModel, Column, Entity, ModelOptionDto};
use features_bakery_model::customer::CustomerData;

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
struct CustomerQueryManager;



pub struct CustomerQuery {}

impl CustomerQuery {
    pub async fn get_by_id<'a>(id: i32) -> Result<CustomerData, DbErr> {
        let model = CustomerQueryManager::get_by_id_i32(id).await?;
        Ok(model.into())
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<CustomerData>, DbErr> {
        let result = CustomerQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
