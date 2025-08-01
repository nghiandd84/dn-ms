use sea_orm::{DbConn, DbErr};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::role::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, RoleForCreateDto, RoleForUpdateDto,
};

use crate::role::util::assign;

#[derive(Mutation)]

#[mutation(key_type(Uuid))]
struct RoleMutationManager {}

pub struct RoleMutation {}

impl RoleMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: RoleForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        RoleMutationManager::create_uuid(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: Uuid,
        data: RoleForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        RoleMutationManager::update_by_id_uuid(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        RoleMutationManager::delete_by_id_uuid(db, id)
    }
}
