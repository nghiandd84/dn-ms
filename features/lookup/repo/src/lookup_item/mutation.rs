use shared_shared_macro::Mutation;

use features_lookup_entities::lookup_item::{
    ActiveModel, Column, Entity, LookupItemForCreateDto, LookupItemForUpdateDto, Model,
    ModelOptionDto,
};

use crate::lookup_item::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct LookupItemMutationManager {}

pub struct LookupItemMutation;

impl LookupItemMutation {
    pub fn create_lookup_item<'a>(
        data: LookupItemForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        LookupItemMutationManager::create_uuid(data.into())
    }

    pub fn update_lookup_item<'a>(
        item_id: Uuid,
        data: LookupItemForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupItemMutationManager::update_by_id_uuid(item_id, data.into())
    }

    pub fn delete_lookup_item<'a>(
        item_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupItemMutationManager::delete_by_id_uuid(item_id)
    }
}
