use sea_orm::ConnectionTrait;
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
        data: AuthCodeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        AuthCodeMutationManager::create_uuid(data.into())
    }

    pub async fn create_with_txn(
        data: AuthCodeForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<(Uuid, String), DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok((result.id, result.code))
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        AuthCodeMutationManager::delete_by_id_uuid(id)
    }
}
