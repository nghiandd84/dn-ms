use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::guest_booking_history::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_booking_model::guest_booking_history::GuestBookingHistoryData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct GuestBookingHistoryQueryManager;

pub struct GuestBookingHistoryQuery;

impl GuestBookingHistoryQuery {
    pub async fn list<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<GuestBookingHistoryData>, AppError> {
        let result = GuestBookingHistoryQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
