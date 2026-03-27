use shared_shared_macro::Mutation;

use features_wallet_entities::top_up_transaction::{
    ActiveModel, TopUpTransactionForCreateDto, TopUpTransactionForUpdateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::top_up_transaction::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TopUpTransactionMutationManager {}

pub struct TopUpTransactionMutation;

impl TopUpTransactionMutation {
    pub fn create_top_up_transaction<'a>(
        data: TopUpTransactionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TopUpTransactionMutationManager::create_uuid(data.into())
    }

    pub fn update_top_up_transaction<'a>(
        top_up_transaction_id: Uuid,
        data: TopUpTransactionForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TopUpTransactionMutationManager::update_by_id_uuid(top_up_transaction_id, data.into())
    }

    pub fn delete_top_up_transaction<'a>(
        top_up_transaction_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TopUpTransactionMutationManager::delete_by_id_uuid(top_up_transaction_id)
    }
}
