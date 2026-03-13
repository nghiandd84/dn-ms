use sea_orm::{DbConn, DbErr};
use shared_shared_config::db::DB_WRITE;
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::access::{
    AccessForCreateDto, AccessForUpdateDto, ActiveModel, Column, Entity, Model, ModelOptionDto,
};

use crate::access::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct AccessMutationManager {}

pub struct AccessMutation {}

impl AccessMutation {
    pub fn create<'a>(
        _db: &'a DbConn,
        data: AccessForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        AccessMutationManager::create_uuid(db, data.into())
    }

    pub fn update<'a>(
        db: &'a DbConn,
        id: Uuid,
        data: AccessForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        AccessMutationManager::update_by_id_uuid(db, id, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete access {:?}", id);
        AccessMutationManager::delete_by_id_uuid(db, id)
    }
}
