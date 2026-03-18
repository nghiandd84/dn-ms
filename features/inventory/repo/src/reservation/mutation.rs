use shared_shared_macro::Mutation;

use features_inventory_entities::reservation::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ReservationForCreateDto,
    ReservationForUpdateDto,
};

use crate::reservation::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ReservationMutationManager {}

pub struct ReservationMutation;

impl ReservationMutation {
    pub fn create_reservation<'a>(
        data: ReservationForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ReservationMutationManager::create_uuid(data.into())
    }

    pub fn update_reservation<'a>(
        event_id: Uuid,
        data: ReservationForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ReservationMutationManager::update_by_id_uuid(event_id, data.into())
    }

    pub fn delete_reservation<'a>(
        event_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ReservationMutationManager::delete_by_id_uuid(event_id)
    }
}
