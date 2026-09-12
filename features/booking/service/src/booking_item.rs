use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking_item::Column;
use features_booking_model::booking_item::{
    BookingItemData, BookingItemForCreateRequest, BookingItemForUpdateRequest,
};
use features_booking_repo::booking_item::{BookingItemMutation, BookingItemQuery};

pub struct BookingItemService {}

impl BookingItemService {
    pub async fn create_booking_item(
        request: BookingItemForCreateRequest,
    ) -> Result<Uuid, AppError> {
        BookingItemMutation::create_booking_item(request.into())
            .await
            .map_err(|e| {
                debug!("Error creating booking item: {:?}", e);
                AppError::Internal("Failed to create booking item".to_string())
            })
    }

    pub async fn get_booking_item_by_id(
        booking_item_id: Uuid,
    ) -> Result<BookingItemData, AppError> {
        BookingItemQuery::get_booking_item_by_id(booking_item_id).await
    }

    pub async fn get_booking_items_by_booking(
        booking_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingItemData>, AppError> {
        let param: FilterParam<String> = FilterParam {
            name: Column::BookingId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(booking_id.to_string()),
            raw_value: booking_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(param)].into();
        BookingItemQuery::get_booking_items(pagination, order, &filters).await
    }

    pub async fn get_booking_items(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingItemData>, AppError> {
        BookingItemQuery::get_booking_items(pagination, order, filters).await
    }

    pub async fn update_booking_item(
        booking_item_id: Uuid,
        request: BookingItemForUpdateRequest,
    ) -> Result<bool, AppError> {
        BookingItemMutation::update_booking_item(booking_item_id, request.into())
            .await
            .map_err(|e| {
                debug!("Error updating booking item: {:?}", e);
                AppError::Internal("Failed to update booking item".to_string())
            })
    }

    pub async fn delete_booking_item(booking_item_id: Uuid) -> Result<bool, AppError> {
        BookingItemMutation::delete_booking_item(booking_item_id)
            .await
            .map_err(|e| {
                debug!("Error deleting booking item: {:?}", e);
                AppError::Internal("Failed to delete booking item".to_string())
            })
    }
}
