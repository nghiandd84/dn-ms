use sea_orm::ConnectionTrait;

use shared_shared_macro::Mutation;

use features_booking_entities::booking::{
    ActiveModel, BookingForCreateDto, BookingForUpdateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::booking::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct BookingMutationManager {}

pub struct BookingMutation;

impl BookingMutation {
    pub fn create_booking<'a>(
        data: BookingForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        BookingMutationManager::create_uuid(data.into())
    }

    /// Insert a booking within an existing transaction, returning the new id.
    pub async fn create_booking_with_txn(
        data: BookingForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }

    pub fn update_booking<'a>(
        booking_id: Uuid,
        data: BookingForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingMutationManager::update_by_id_uuid(booking_id, data.into())
    }

    /// Apply an update to a booking within an existing transaction.
    pub async fn update_booking_with_txn(
        booking_id: Uuid,
        data: BookingForUpdateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<bool, DbErr> {
        let existing = Entity::find_by_id(booking_id).one(txn).await?;
        let Some(existing) = existing else {
            return Ok(false);
        };
        let option_dto: ModelOptionDto = data.into();
        let active_model = assign(existing.into(), option_dto);
        active_model.update(txn).await?;
        Ok(true)
    }

    pub fn delete_booking<'a>(
        booking_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingMutationManager::delete_by_id_uuid(booking_id)
    }
}
