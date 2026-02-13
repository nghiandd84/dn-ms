use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_booking_entities::booking::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, BookingForCreateDto, BookingForUpdateDto,
};

use crate::booking::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct BookingMutationManager {}

pub struct BookingMutation;

impl BookingMutation {
    pub fn create_booking<'a>(
        db: &'a DbConn,
        data: BookingForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        BookingMutationManager::create_uuid(db, data.into())
    }

    pub fn update_booking<'a>(
        db: &'a DbConn,
        booking_id: Uuid,
        data: BookingForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingMutationManager::update_by_id_uuid(db, booking_id, data.into())
    }

    pub fn delete_booking<'a>(
        db: &'a DbConn,
        booking_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BookingMutationManager::delete_by_id_uuid(db, booking_id)
    }
}
