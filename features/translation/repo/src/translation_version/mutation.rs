use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_translation_entities::translation_version::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TranslationVersionForCreateDto, TranslationVersionForUpdateDto,
};

use crate::translation_version::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TranslationVersionMutationManager {}

pub struct TranslationVersionMutation {}

impl TranslationVersionMutation {
    pub fn create_translation_version<'a>(
        db: &'a DbConn,
        data: TranslationVersionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TranslationVersionMutationManager::create_uuid(db, data.into())
    }

    pub fn update_translation_version<'a>(
        db: &'a DbConn,
        version_id: Uuid,
        data: TranslationVersionForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TranslationVersionMutationManager::update_by_id_uuid(db, version_id, data.into())
    }

    pub fn delete_translation_version<'a>(
        db: &'a DbConn,
        version_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TranslationVersionMutationManager::delete_by_id_uuid(db, version_id)
    }
}
