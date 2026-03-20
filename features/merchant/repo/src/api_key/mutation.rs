use shared_shared_macro::Mutation;

use features_merchant_entities::api_key::{
    ActiveModel, ApiKeyForCreateDto, ApiKeyForUpdateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::api_key::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct ApiKeyMutationManager {}

impl ApiKeyMutationManager {}

pub struct ApiKeyMutation;

impl ApiKeyMutation {
    pub fn create_api_key<'a>(
        data: ApiKeyForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        ApiKeyMutationManager::create_i32(data.into())
    }

    pub fn bulk_create_api_keys<'a>(
        data: Vec<ApiKeyForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>> + 'a {
        ApiKeyMutationManager::bulk_create_i32(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_api_key<'a>(
        api_key_id: i32,
        data: ApiKeyForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ApiKeyMutationManager::update_by_id_i32(api_key_id, data.into())
    }

    pub fn bulk_update_api_keys<'a>(
        data: Vec<(i32, ApiKeyForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>> + 'a {
        ApiKeyMutationManager::bulk_update_by_id_i32(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_api_key<'a>(
        api_key_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ApiKeyMutationManager::delete_by_id_i32(api_key_id)
    }
}
