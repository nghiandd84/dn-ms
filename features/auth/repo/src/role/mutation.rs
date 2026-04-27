use tracing::debug;

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
        data: RoleForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        RoleMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: RoleForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        RoleMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        RoleMutationManager::delete_by_id_uuid(id)
    }
}
