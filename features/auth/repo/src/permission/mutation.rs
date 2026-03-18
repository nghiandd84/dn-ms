
use tracing::debug;

use shared_shared_macro::Mutation;

use features_auth_entities::permission::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PermissionForCreateDto,
    PermissionForCreateRequestDto,
};

use crate::permission::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PermissionMutationManager {}

pub struct PermissionMutation {}

impl PermissionMutation {
    pub fn create<'a>(
        data: PermissionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        debug!("Create permission {:?}", data);
        PermissionMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: PermissionForCreateRequestDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Update  permission {:?}", data);
        PermissionMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        PermissionMutationManager::delete_by_id_uuid(id)
    }
}
