use shared_shared_macro::Mutation;

use features_wallet_entities::transaction::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, TransactionForCreateDto,
    TransactionForUpdateDto,
};

use crate::transaction::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct TransactionMutationManager {}

pub struct TransactionMutation;

impl TransactionMutation {
    pub fn create_transaction<'a>(
        data: TransactionForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        TransactionMutationManager::create_uuid(data.into())
    }

    pub fn update_transaction<'a>(
        transaction_id: Uuid,
        data: TransactionForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TransactionMutationManager::update_by_id_uuid(transaction_id, data.into())
    }

    pub fn delete_transaction<'a>(
        transaction_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        TransactionMutationManager::delete_by_id_uuid(transaction_id)
    }
}
