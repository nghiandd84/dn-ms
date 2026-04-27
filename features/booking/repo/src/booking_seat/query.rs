use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::booking_seat::{ActiveModel, Column, Entity, ModelOptionDto};
use features_booking_model::booking_seat::BookingSeatData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct BookingSeatQueryManager;

pub struct BookingSeatQuery;

impl BookingSeatQuery {
    pub async fn get_booking_seat_by_id(
        booking_seat_id: Uuid,
    ) -> Result<BookingSeatData, AppError> {
        let model = BookingSeatQueryManager::get_by_id_uuid(booking_seat_id).await?;
        Ok(model.into())
    }

    pub async fn get_booking_seats<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<BookingSeatData>, AppError> {
        let result = BookingSeatQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
