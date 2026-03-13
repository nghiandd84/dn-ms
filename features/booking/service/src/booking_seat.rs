use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking_seat::Column;
use features_booking_model::booking_seat::{
    BookingSeatData, BookingSeatForCreateRequest, BookingSeatForUpdateRequest,
};
use features_booking_repo::booking_seat::{BookingSeatMutation, BookingSeatQuery};

pub struct BookingSeatService {}

impl BookingSeatService {
    pub async fn create_booking_seat<'a>(
        booking_seat_request: BookingSeatForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let booking_seat_id =
            BookingSeatMutation::create_booking_seat(booking_seat_request.into()).await;
        let id = match booking_seat_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating booking seat: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create booking seat".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_booking_seat_by_id<'a>(
        booking_seat_id: Uuid,
    ) -> Result<BookingSeatData, AppError> {
        BookingSeatQuery::get_booking_seat_by_id(booking_seat_id).await
    }

    pub async fn get_booking_seats_by_booking(
        booking_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingSeatData>, AppError> {
        let booking_id_column = Column::BookingId.to_string();
        let param: FilterParam<String> = FilterParam {
            name: booking_id_column,
            operator: FilterOperator::Equal,
            value: Some(booking_id.to_string()),
            raw_value: booking_id.to_string(),
        };
        let booking_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![booking_filter];
        BookingSeatQuery::get_booking_seats(&pagination, &order, &filters).await
    }

    pub async fn get_booking_seats<'a>(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingSeatData>, AppError> {
        BookingSeatQuery::get_booking_seats(pagination, order, filters).await
    }

    pub async fn update_booking_seat(
        booking_seat_id: Uuid,
        booking_seat_request: BookingSeatForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result =
            BookingSeatMutation::update_booking_seat(booking_seat_id, booking_seat_request.into())
                .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating booking seat: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update booking seat".to_string(),
                ))
            }
        }
    }

    pub async fn delete_booking_seat(booking_seat_id: Uuid) -> Result<bool, AppError> {
        let result = BookingSeatMutation::delete_booking_seat(booking_seat_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting booking seat: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete booking seat".to_string(),
                ))
            }
        }
    }
}
