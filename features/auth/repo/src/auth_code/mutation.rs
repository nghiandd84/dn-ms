use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::auth_code::{
    ActiveModel, AuthCodeForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::auth_code::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct AuthCodeMutationManager {}

pub struct AuthCodeMutation {}

impl AuthCodeMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: AuthCodeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        AuthCodeMutationManager::create_uuid(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        AuthCodeMutationManager::delete_by_id_uuid(db, id)
    }
}
