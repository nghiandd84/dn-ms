use sea_orm::{DbConn, DbErr};
use serde_json::de;
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::client::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ClientForCreateDto, ClientForUpdateDto,
};

use crate::client::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ClientMutationManager {}

pub struct ClientMutation {}

impl ClientMutation {
    pub fn create<'a>(
        db: &'a DbConn,
        data: ClientForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ClientMutationManager::create_uuid(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: Uuid,
        data: ClientForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Update scope {:?} data {:?}", id, data);
        ClientMutationManager::update_by_id_uuid(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete scope {:?}", id);
        ClientMutationManager::delete_by_id_uuid(db, id)
    }
}
