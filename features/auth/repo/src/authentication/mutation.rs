use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::authentication::{
    ActiveModel, AuthenticationRequestForCreateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::authentication::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct AuthenticationMutationManager {}

pub struct AuthenticationRequestMutation {}

impl AuthenticationRequestMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: AuthenticationRequestForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        AuthenticationMutationManager::create_uuid(db, data.into())
    }
}
