use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::active_code::{
    ActiveCodeForCreateDto, ActiveCodeForUpdateDto, ActiveModel, Column, Entity, Model,
    ModelOptionDto,
};

use crate::active_code::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ActiveCodeMutationManager {}

pub struct ActiveCodeMutation {}

impl ActiveCodeMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: ActiveCodeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ActiveCodeMutationManager::create_uuid(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: Uuid,
        data: ActiveCodeForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ActiveCodeMutationManager::update_by_id_uuid(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ActiveCodeMutationManager::delete_by_id_uuid(db, id)
    }
}
