use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::booking::{ActiveModel, Column, Entity, ModelOptionDto};
use features_booking_model::booking::BookingData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct BookingQueryManager;

pub struct BookingQuery;

impl BookingQuery {
    pub async fn get_booking_by_id(booking_id: Uuid) -> Result<BookingData, AppError> {
        let model = BookingQueryManager::get_by_id_uuid(booking_id).await?;
        Ok(model.into())
    }

    pub async fn get_bookings<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let result = BookingQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
