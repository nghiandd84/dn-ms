use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::booking_item::{ActiveModel, Column, Entity, ModelOptionDto};
use features_booking_model::booking_item::BookingItemData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct BookingItemQueryManager;

pub struct BookingItemQuery;

impl BookingItemQuery {
    pub async fn get_booking_item_by_id(booking_item_id: Uuid) -> Result<BookingItemData, AppError> {
        let model = BookingItemQueryManager::get_by_id_uuid(booking_item_id).await?;
        Ok(model.into())
    }

    pub async fn get_booking_items<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<BookingItemData>, AppError> {
        let result = BookingItemQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
