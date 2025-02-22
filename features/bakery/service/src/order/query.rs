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

impl OrderQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct OrderQuery {}

impl OrderQuery {
    pub async fn get_by_id<'a>(db: &'a DbConn, id: i32) -> Result<OrderData, DbErr> {
        let model = OrderQueryManager::get_by_id_i32(db, id).await?;
        Ok(model.into())
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<OrderData>, DbErr> {
        let result = OrderQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
