use shared_shared_macro::Mutation;

use features_bakery_entities::bakery::{
    ActiveModel, BakeryForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct BakeryMutationManager {}

pub struct BakeryMutation {}

impl BakeryMutation {
    pub fn create<'a>(
        data: BakeryForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        BakeryMutationManager::create_i32(data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        BakeryMutationManager::delete_by_id_i32(id)
    }
}
