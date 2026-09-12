use sea_orm::ConnectionTrait;

use shared_shared_macro::Mutation;

use features_booking_entities::booking_item::{
    ActiveModel, BookingItemForCreateDto, BookingItemForUpdateDto, Column, Entity, Model,
    ModelOptionDto,
};

use crate::booking_item::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct BookingItemMutationManager {}

pub struct BookingItemMutation;

impl BookingItemMutation {
    pub fn create_booking_item<'a>(
        data: BookingItemForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        BookingItemMutationManager::create_uuid(data.into())
    }

    /// Insert a booking item within an existing transaction.
    pub async fn create_booking_item_with_txn(
        data: BookingItemForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }

    pub fn update_booking_item<'a>(
        booking_item_id: Uuid,
        data: BookingItemForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingItemMutationManager::update_by_id_uuid(booking_item_id, data.into())
    }

    pub fn delete_booking_item<'a>(
        booking_item_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingItemMutationManager::delete_by_id_uuid(booking_item_id)
    }
}
