use sea_orm::{DbConn, DbErr};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_auth_entities::scope::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ScopeForCreateDto, ScopeForUpdateDto,
};

use crate::scope::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ScopeMutationManager {}

pub struct ScopeMutation {}

impl ScopeMutation {
    pub fn create<'a>(
        data: ScopeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ScopeMutationManager::create_uuid(data.into())
    }

    pub fn update<'a>(
        id: Uuid,
        data: ScopeForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete scope {:?} data {:?}", id, data);
        ScopeMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        debug!("Delete scope {:?}", id);
        ScopeMutationManager::delete_by_id_uuid(id)
    }
}
