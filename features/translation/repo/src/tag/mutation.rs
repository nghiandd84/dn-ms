use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_translation_entities::tag::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TagForCreateDto, TagForUpdateDto,
};

use crate::tag::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TagMutationManager {}

pub struct TagMutation {}

impl TagMutation {
    pub fn create_tag<'a>(
        db: &'a DbConn,
        data: TagForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TagMutationManager::create_uuid(db, data.into())
    }

    pub fn update_tag<'a>(
        db: &'a DbConn,
        tag_id: Uuid,
        data: TagForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagMutationManager::update_by_id_uuid(db, tag_id, data.into())
    }

    pub fn delete_tag<'a>(
        db: &'a DbConn,
        tag_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TagMutationManager::delete_by_id_uuid(db, tag_id)
    }
}
