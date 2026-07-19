use tracing::debug;

use shared_shared_macro::Mutation;

use features_auth_entities::field_permission::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, FieldPermissionForCreateDto,
    FieldPermissionForUpdateDto,
};

use crate::field_permission::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct FieldPermissionMutationManager {}

pub struct FieldPermissionMutation {}

impl FieldPermissionMutation {
    pub fn create<'a>(
        data: FieldPermissionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        debug!("Create field_permission {:?}", data);
        FieldPermissionMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: FieldPermissionForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Update field_permission {:?}", data);
        FieldPermissionMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete field_permission {:?}", id);
        FieldPermissionMutationManager::delete_by_id_uuid(id)
    }
}
