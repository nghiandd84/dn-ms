use shared_shared_macro::Mutation;

use features_email_template_entities::template_translations::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TemplateTranslationForCreateDto,
    TemplateTranslationForUpdateDto,
};

use crate::template_translation::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct TemplateTranslationMutationManager {}

pub struct TemplateTranslationMutation {}

impl TemplateTranslationMutation {
    pub fn create<'a>(
        data: TemplateTranslationForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        TemplateTranslationMutationManager::create_i32(data.into())
    }

    pub fn update<'a>(
        id: i32,
        data: TemplateTranslationForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplateTranslationMutationManager::update_by_id_i32(id, data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplateTranslationMutationManager::delete_by_id_i32(id)
    }
}
