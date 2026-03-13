use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_booking_entities::booking_seat::{
    ActiveModel, BookingSeatForCreateDto, BookingSeatForUpdateDto, Column, Entity, Model,
    ModelOptionDto,
};

use crate::booking_seat::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct BookingSeatMutationManager {}

pub struct BookingSeatMutation;

impl BookingSeatMutation {
    pub fn create_booking_seat<'a>(
        data: BookingSeatForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        BookingSeatMutationManager::create_uuid(data.into())
    }

    pub fn update_booking_seat<'a>(
        booking_seat_id: Uuid,
        data: BookingSeatForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingSeatMutationManager::update_by_id_uuid(booking_seat_id, data.into())
    }

    pub fn delete_booking_seat<'a>(
        booking_seat_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingSeatMutationManager::delete_by_id_uuid(booking_seat_id)
    }
}
