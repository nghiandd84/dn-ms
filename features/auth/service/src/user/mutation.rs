use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::user::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, UserForCreateDto, UserForUpdateProfileDto,
};

use crate::user::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct UserMutationManager {}

pub struct UserMutation {}

impl UserMutation {
    pub fn create_user<'a>(
        db: &'a DbConn,
        data: UserForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        (&UserMutationManager::create_uuid)(db, data.into())
    }

    pub fn update_profile<'a>(
        db: &'a DbConn,
        user_id: Uuid,
        data: UserForUpdateProfileDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserMutationManager::update_by_id_uuid(db, user_id, data.into())
    }

    pub fn delete_user<'a>(
        db: &'a DbConn,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserMutationManager::delete_by_id_uuid(db, user_id)
    }
}
