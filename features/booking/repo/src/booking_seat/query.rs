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

impl BookingSeatQueryManager {
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

pub struct BookingSeatQuery;

impl BookingSeatQuery {
    pub async fn get_booking_seat_by_id(
        db: &DbConn,
        booking_seat_id: Uuid,
    ) -> Result<BookingSeatData, AppError> {
        let model = BookingSeatQueryManager::get_by_id_uuid(db, booking_seat_id).await?;
        Ok(model.into())
    }

    pub async fn get_booking_seats<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<BookingSeatData>, AppError> {
        let result = BookingSeatQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
