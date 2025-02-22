use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_bakery_entities::order::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, OrderForCreateDto,
};

use super::util::assign;

#[derive(Mutation)]
#[mutation(key_type(i32))]
struct OrderMutationManager {}

pub struct OrderMutation {}

impl OrderMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: OrderForCreateDto,
    ) -> impl std::future::Future<Output = Result<i32, DbErr>> + 'a {
        OrderMutationManager::create_i32(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        OrderMutationManager::delete_by_id_i32(db, id)
    }
}
