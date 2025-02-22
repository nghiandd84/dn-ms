use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_bakery_entities::lineitem::{
    ActiveModel, Column, Entity, LineitemForCreateDto, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct LineitemMutationManager {}

pub struct LineitemMutation {}

impl LineitemMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: LineitemForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        LineitemMutationManager::create_i32(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LineitemMutationManager::delete_by_id_i32(db, id)
    }
}
