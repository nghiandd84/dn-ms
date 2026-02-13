use sea_orm::{DbConn, Iden};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking::Column;
use features_booking_model::booking::{BookingData, BookingForCreateRequest, BookingForUpdateRequest};
use features_booking_repo::booking::{BookingMutation, BookingQuery};

pub struct BookingService {}

impl BookingService {
    pub async fn create_booking<'a>(
        db: &'a DbConn,
        booking_request: BookingForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let booking_id = BookingMutation::create_booking(db, booking_request.into()).await;
        let id = match booking_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating booking: {:?}", e);
                return Err(AppError::Internal("Failed to create booking".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_booking_by_id<'a>(db: &'a DbConn, booking_id: Uuid) -> Result<BookingData, AppError> {
        BookingQuery::get_booking_by_id(db, booking_id).await
    }

    pub async fn get_bookings_by_status(
        db: &DbConn,
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        BookingQuery::get_bookings(db, &pagination, &order, &filters).await
    }

    pub async fn get_bookings_by_user(
        db: &DbConn,
        user_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let user_id_column = Column::UserId.to_string();
        let param: FilterParam<String> = FilterParam {
            name: user_id_column,
            operator: FilterOperator::Equal,
            value: Some(user_id.to_string()),
            raw_value: user_id.to_string(),
        };
        let user_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![user_filter];
        BookingQuery::get_bookings(db, &pagination, &order, &filters).await
    }

    pub async fn get_bookings<'a>(
        db: &'a DbConn,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingData>, AppError> {
        BookingQuery::get_bookings(db, pagination, order, filters).await
    }

    pub async fn update_booking(
        db: &DbConn,
        booking_id: Uuid,
        booking_request: BookingForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = BookingMutation::update_booking(db, booking_id, booking_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating booking: {:?}", e);
                Err(AppError::Internal("Failed to update booking".to_string()))
            }
        }
    }

    pub async fn delete_booking(db: &DbConn, booking_id: Uuid) -> Result<bool, AppError> {
        let result = BookingMutation::delete_booking(db, booking_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting booking: {:?}", e);
                Err(AppError::Internal("Failed to delete booking".to_string()))
            }
        }
    }
}
