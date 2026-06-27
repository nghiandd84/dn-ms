use sea_orm::ConnectionTrait;
use shared_shared_macro::Mutation;

use features_auth_entities::user::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, UserForCreateDto, UserForUpdateDto,
};

use crate::user::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct UserMutationManager {}

pub struct UserMutation {}

impl UserMutation {
    pub fn create_user<'a>(
        data: UserForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        (&UserMutationManager::create_uuid)(data.into())
    }

    pub async fn create_user_with_txn(
        data: UserForCreateDto,
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
        data: UserForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete_user<'a>(
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserMutationManager::delete_by_id_uuid(user_id)
    }
}
