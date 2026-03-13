
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_bakery_entities::cake::{
    ActiveModel, CakeForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct CakeMutationManager {}

pub struct CakeMutation {}

impl CakeMutation {
    pub fn create<'a>(
        data: CakeForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        CakeMutationManager::create_i32(data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CakeMutationManager::delete_by_id_i32(id)
    }
}
