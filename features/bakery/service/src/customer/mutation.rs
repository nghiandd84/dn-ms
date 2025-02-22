use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_bakery_entities::customer::{
    ActiveModel, Column, CustomerForCreateDto, Entity, Model, ModelOptionDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct CustomerMutationManager {}

pub struct CustomerMutation {}

impl CustomerMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: CustomerForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        CustomerMutationManager::create_i32(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CustomerMutationManager::delete_by_id_i32(db, id)
    }
}
