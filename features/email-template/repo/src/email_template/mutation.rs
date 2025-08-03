use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

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
        db: &'a DbConn,
        data: EmailTemplateForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        EmailTemplateMutationManager::create_i32(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: i32,
        data: EmailTemplateForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EmailTemplateMutationManager::update_by_id_i32(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EmailTemplateMutationManager::delete_by_id_i32(db, id)
    }
}
