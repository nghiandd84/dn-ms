use sea_orm::ConnectionTrait;

use shared_shared_macro::Mutation;

use features_booking_entities::guest_booking_item::{
    ActiveModel, Column, Entity, GuestBookingItemForCreateDto, GuestBookingItemForUpdateDto, Model,
    ModelOptionDto,
};

use crate::guest_booking_item::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct GuestBookingItemMutationManager {}

pub struct GuestBookingItemMutation;

impl GuestBookingItemMutation {
    pub fn create_guest_booking_item<'a>(
        data: GuestBookingItemForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        GuestBookingItemMutationManager::create_uuid(data.into())
    }

    /// Insert a guest booking item within an existing transaction.
    pub async fn create_guest_booking_item_with_txn(
        data: GuestBookingItemForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }

    pub fn update_guest_booking_item<'a>(
        guest_booking_item_id: Uuid,
        data: GuestBookingItemForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        GuestBookingItemMutationManager::update_by_id_uuid(guest_booking_item_id, data.into())
    }

    pub fn delete_guest_booking_item<'a>(
        guest_booking_item_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        GuestBookingItemMutationManager::delete_by_id_uuid(guest_booking_item_id)
    }
}
