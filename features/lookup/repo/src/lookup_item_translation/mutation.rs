use shared_shared_macro::Mutation;

use features_lookup_entities::lookup_item_translation::{
    ActiveModel, Column, Entity, LookupItemTranslationForCreateDto,
    LookupItemTranslationForUpdateDto, Model, ModelOptionDto,
};

use crate::lookup_item_translation::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct LookupItemTranslationMutationManager {}

pub struct LookupItemTranslationMutation;

impl LookupItemTranslationMutation {
    pub fn create_translation<'a>(
        data: LookupItemTranslationForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        LookupItemTranslationMutationManager::create_uuid(data.into())
    }

    pub fn update_translation<'a>(
        lookup_item_translation_id: Uuid,
        data: LookupItemTranslationForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupItemTranslationMutationManager::update_by_id_uuid(
            lookup_item_translation_id,
            data.into(),
        )
    }

    pub fn delete_translation<'a>(
        lookup_item_translation_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupItemTranslationMutationManager::delete_by_id_uuid(lookup_item_translation_id)
    }
}
