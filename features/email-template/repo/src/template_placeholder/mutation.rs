use shared_shared_macro::Mutation;

use features_email_template_entities::template_placeholders::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TemplatePlaceholderForCreateDto,
    TemplatePlaceholderForUpdateDto,
};

use crate::template_placeholder::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct TemplatePlaceholderMutationManager {}

pub struct TemplatePlaceholderMutation {}

impl TemplatePlaceholderMutation {
    pub fn create<'a>(
        data: TemplatePlaceholderForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::create_i32(data.into())
    }

    pub fn update<'a>(
        id: i32,
        data: TemplatePlaceholderForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::update_by_id_i32(id, data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::delete_by_id_i32(id)
    }
}
