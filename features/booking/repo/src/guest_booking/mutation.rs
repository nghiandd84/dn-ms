use sea_orm::ConnectionTrait;

use shared_shared_macro::Mutation;

use features_booking_entities::guest_booking::{
    ActiveModel, Column, Entity, GuestBookingForCreateDto, GuestBookingForUpdateDto, Model,
    ModelOptionDto,
};

use crate::guest_booking::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct GuestBookingMutationManager {}

pub struct GuestBookingMutation;

impl GuestBookingMutation {
    pub fn create_guest_booking<'a>(
        data: GuestBookingForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        GuestBookingMutationManager::create_uuid(data.into())
    }

    /// Insert a guest booking within an existing transaction, returning the new id.
    pub async fn create_guest_booking_with_txn(
        data: GuestBookingForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }

    pub fn update_guest_booking<'a>(
        guest_booking_id: Uuid,
        data: GuestBookingForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        GuestBookingMutationManager::update_by_id_uuid(guest_booking_id, data.into())
    }

    /// Apply an update to a guest booking within an existing transaction.
    pub async fn update_guest_booking_with_txn(
        guest_booking_id: Uuid,
        data: GuestBookingForUpdateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<bool, DbErr> {
        let existing = Entity::find_by_id(guest_booking_id).one(txn).await?;
        let Some(existing) = existing else {
            return Ok(false);
        };
        let option_dto: ModelOptionDto = data.into();
        let active_model = assign(existing.into(), option_dto);
        active_model.update(txn).await?;
        Ok(true)
    }

    pub fn delete_guest_booking<'a>(
        guest_booking_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        GuestBookingMutationManager::delete_by_id_uuid(guest_booking_id)
    }
}
