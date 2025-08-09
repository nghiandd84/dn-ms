use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

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
        db: &'a DbConn,
        data: TemplatePlaceholderForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::create_i32(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: i32,
        data: TemplatePlaceholderForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::update_by_id_i32(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TemplatePlaceholderMutationManager::delete_by_id_i32(db, id)
    }
}
