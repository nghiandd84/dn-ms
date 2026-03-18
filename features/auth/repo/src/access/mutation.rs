use shared_shared_macro::Mutation;

use features_auth_entities::access::{
    AccessForCreateDto, AccessForUpdateDto, ActiveModel, Column, Entity, Model, ModelOptionDto,
};

use crate::access::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct AccessMutationManager {}

pub struct AccessMutation {}

impl AccessMutation {
    pub fn create<'a>(
        data: AccessForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        AccessMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: AccessForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        AccessMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        AccessMutationManager::delete_by_id_uuid(id)
    }
}
