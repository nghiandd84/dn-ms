use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_bakery_entities::baker::{
    ActiveModel, BakerForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct BakerMutationManager {}

pub struct BakerMutation {}

impl BakerMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: BakerForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        BakerMutationManager::create_i32(db, data.into())
    }
    /*
    pub fn update<'a>(
        db: &'a DbConn,
        id: Uuid,
        data: RoleForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        RoleMutationManager::update_by_id_uuid(db, id, data.into())
    }
    */
    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BakerMutationManager::delete_by_id_i32(db, id)
    }
}
