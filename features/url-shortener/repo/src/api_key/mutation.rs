use shared_shared_macro::Mutation;

use features_url_shortener_entities::api_key::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ApiKeyForCreateDto,
};

use crate::api_key::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ApiKeyMutationManager {}

pub struct ApiKeyMutation;

impl ApiKeyMutation {
    pub fn create_api_key<'a>(
        data: ApiKeyForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ApiKeyMutationManager::create_uuid(data.into())
    }

    pub fn delete_api_key<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ApiKeyMutationManager::delete_by_id_uuid(id)
    }
}
