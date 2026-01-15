use sea_orm::{DbConn, DbErr};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::permission::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PermissionForCreateDto,
};

use crate::permission::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PermissionMutationManager {}

pub struct PermissionMutation {}

impl PermissionMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: PermissionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        debug!("Create permission {:?}", data);
        PermissionMutationManager::create_uuid(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        PermissionMutationManager::delete_by_id_uuid(db, id)
    }
}
