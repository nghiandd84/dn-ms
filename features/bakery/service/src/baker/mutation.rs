use shared_shared_macro::Mutation;

use features_bakery_entities::baker::{
    ActiveModel, BakerForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct BakerMutationManager {}

pub struct BakerMutation {}

impl BakerMutation {
    pub fn create<'a>(
        data: BakerForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        BakerMutationManager::create_i32(data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BakerMutationManager::delete_by_id_i32(id)
    }
}
