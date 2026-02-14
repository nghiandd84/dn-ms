use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_inventory_entities::seat::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, SeatForCreateDto, SeatForUpdateDto,
};

use crate::seat::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct SeatMutationManager {}

pub struct SeatMutation;

impl SeatMutation {
    pub fn create_seat<'a>(
        db: &'a DbConn,
        data: SeatForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        SeatMutationManager::create_uuid(db, data.into())
    }

    pub fn bulk_create_seats<'a>(
        db: &'a DbConn,
        data: Vec<SeatForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        SeatMutationManager::bulk_create_uuid(db, data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_seat<'a>(
        db: &'a DbConn,
        event_id: Uuid,
        data: SeatForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        SeatMutationManager::update_by_id_uuid(db, event_id, data.into())
    }

    pub fn bulk_update_seats<'a>(
        db: &'a DbConn,
        data: Vec<(Uuid, SeatForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        SeatMutationManager::bulk_update_by_id_uuid(
            db,
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_seat<'a>(
        db: &'a DbConn,
        event_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        SeatMutationManager::delete_by_id_uuid(db, event_id)
    }
}
