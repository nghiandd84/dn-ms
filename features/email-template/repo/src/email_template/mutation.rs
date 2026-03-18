use shared_shared_macro::Mutation;

use features_email_template_entities::email_templates::{
    ActiveModel, Column, EmailTemplateForCreateDto, EmailTemplateForUpdateDto, Entity, Model,
    ModelOptionDto,
};

use crate::email_template::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct EmailTemplateMutationManager {}

pub struct EmailTemplateMutation {}

impl EmailTemplateMutation {
    pub fn create<'a>(
        data: EmailTemplateForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        EmailTemplateMutationManager::create_i32(data.into())
    }

    pub fn update<'a>(
        id: i32,
        data: EmailTemplateForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EmailTemplateMutationManager::update_by_id_i32(id, data.into())
    }

    pub fn delete<'a>(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EmailTemplateMutationManager::delete_by_id_i32(id)
    }
}
