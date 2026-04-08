use shared_shared_macro::Mutation;

use features_auth_entities::client::{
    ActiveModel, ClientForCreateDto, ClientForUpdateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::client::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ClientMutationManager {}

pub struct ClientMutation {}

impl ClientMutation {
    pub fn create<'a>(
        data: ClientForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ClientMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: ClientForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ClientMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ClientMutationManager::delete_by_id_uuid(id)
    }
}
