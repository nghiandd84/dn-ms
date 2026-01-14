use sea_orm::{DbConn, DbErr};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::role_permission::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, RolePermissionForCreateDto,
};

use crate::role_permission::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct RolePermissionMutationManager {}

pub struct RolePermissionMutation {}

impl RolePermissionMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: RolePermissionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        RolePermissionMutationManager::create_uuid(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        RolePermissionMutationManager::delete_by_id_uuid(db, id)
    }
}
