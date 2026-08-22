use shared_shared_macro::Mutation;

use features_tagging_entities::tag_group::{
    ActiveModel, Column, Entity, TagGroupForCreateDto, TagGroupForUpdateDto, Model, ModelOptionDto,
};

use crate::tag_group::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TagGroupMutationManager {}

pub struct TagGroupMutation;

impl TagGroupMutation {
    pub fn create_tag_group<'a>(
        data: TagGroupForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TagGroupMutationManager::create_uuid(data.into())
    }

    pub fn update_tag_group<'a>(
        id: Uuid,
        data: TagGroupForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagGroupMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete_tag_group<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagGroupMutationManager::delete_by_id_uuid(id)
    }
}
