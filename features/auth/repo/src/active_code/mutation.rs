use sea_orm::ConnectionTrait;
use shared_shared_macro::Mutation;

use features_auth_entities::active_code::{
    ActiveCodeForCreateDto, ActiveCodeForUpdateDto, ActiveModel, Column, Entity, Model,
    ModelOptionDto,
};

use crate::active_code::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ActiveCodeMutationManager {}

pub struct ActiveCodeMutation {}

impl ActiveCodeMutation {
    pub fn create<'a>(
        data: ActiveCodeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ActiveCodeMutationManager::create_uuid(data.into())
    }

    pub async fn create_with_txn(
        data: ActiveCodeForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model: ActiveModel = model.into();
        active_model.not_set(Column::Id);
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }

    pub fn update<'a>(
        id: Uuid,
        data: ActiveCodeForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ActiveCodeMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete<'a>(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ActiveCodeMutationManager::delete_by_id_uuid(id)
    }
}
