use sea_orm::{DbConn, DbErr};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::token::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TokenForCreateDto,
};

use crate::token::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TokenMutationManager {}

pub struct TokenMutation {}

impl TokenMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        mut data: TokenForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        // TODO create token data from code
        debug!("Create token with data: {:?}", data);
        data.code = None; // Clear code as it is not needed in the token
        TokenMutationManager::create_uuid(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete token {:?}", id);
        TokenMutationManager::delete_by_id_uuid(db, id)
    }
}
