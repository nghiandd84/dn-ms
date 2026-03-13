use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_translation_entities::translation_key::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TranslationKeyForCreateDto,
    TranslationKeyForUpdateDto,
};

use crate::translation_key::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TranslationKeyMutationManager {}

pub struct TranslationKeyMutation {}

impl TranslationKeyMutation {
    pub fn create_translation_key<'a>(
        data: TranslationKeyForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TranslationKeyMutationManager::create_uuid(data.into())
    }

    pub fn update_translation_key<'a>(
        key_id: Uuid,
        data: TranslationKeyForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TranslationKeyMutationManager::update_by_id_uuid(key_id, data.into())
    }

    pub fn delete_translation_key<'a>(
        key_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TranslationKeyMutationManager::delete_by_id_uuid(key_id)
    }
}
