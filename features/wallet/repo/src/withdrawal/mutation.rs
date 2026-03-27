use shared_shared_macro::Mutation;

use features_wallet_entities::withdrawal::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, WithdrawalForCreateDto,
    WithdrawalForUpdateDto,
};

use crate::withdrawal::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct WithdrawalMutationManager {}

pub struct WithdrawalMutation;

impl WithdrawalMutation {
    pub fn create_withdrawal<'a>(
        data: WithdrawalForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        WithdrawalMutationManager::create_uuid(data.into())
    }

    pub fn update_withdrawal<'a>(
        withdrawal_id: Uuid,
        data: WithdrawalForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WithdrawalMutationManager::update_by_id_uuid(withdrawal_id, data.into())
    }

    pub fn delete_withdrawal<'a>(
        withdrawal_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WithdrawalMutationManager::delete_by_id_uuid(withdrawal_id)
    }
}
