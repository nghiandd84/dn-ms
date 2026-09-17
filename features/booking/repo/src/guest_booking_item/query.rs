use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::guest_booking_item::{
    ActiveModel, Column, Entity, Model, ModelOptionDto,
};
use features_booking_model::guest_booking_item::GuestBookingItemData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct GuestBookingItemQueryManager;

pub struct GuestBookingItemQuery;

impl GuestBookingItemQuery {
    pub async fn get_guest_booking_item_by_id(
        guest_booking_item_id: Uuid,
    ) -> Result<GuestBookingItemData, AppError> {
        let model = GuestBookingItemQueryManager::get_by_id_uuid(guest_booking_item_id).await?;
        Ok(model.into())
    }

    pub async fn get_guest_booking_items<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<GuestBookingItemData>, AppError> {
        let result = GuestBookingItemQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    /// List all seat items for a guest booking within an existing transaction.
    /// Returns raw entity models (used by the promotion flow).
    pub async fn list_by_guest_booking_with_txn(
        guest_booking_id: Uuid,
        txn: &impl sea_orm::ConnectionTrait,
    ) -> Result<Vec<Model>, sea_orm::DbErr> {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
        Entity::find()
            .filter(Column::GuestBookingId.eq(guest_booking_id))
            .all(txn)
            .await
    }
}
