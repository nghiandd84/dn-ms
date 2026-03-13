
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
        data: RolePermissionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        RolePermissionMutationManager::create_uuid(data.into())
    }

    pub async fn assign_permissions<'a>(
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<bool, DbErr> {
        debug!(
            "Assign permissions {:?} to role {:?}",
            permission_ids, role_id
        );
        for permission_id in &permission_ids {
            let create_request = RolePermissionForCreateDto {
                role_id,
                permission_id: *permission_id,
            };
            let insert = RolePermissionMutationManager::create_uuid(create_request.into()).await;
            if insert.is_err() {
                debug!(
                    "Failed to assign permission {:?} to role {:?}: {:?}",
                    permission_id,
                    role_id,
                    insert.err()
                );
            }
        }
        Ok(true)
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete role {:?}", id);
        RolePermissionMutationManager::delete_by_id_uuid(id)
    }
}
