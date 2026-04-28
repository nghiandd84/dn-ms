use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_model::transaction::{
    TransactionData, TransactionForCreateRequest, TransactionForUpdateRequest,
};
use features_wallet_repo::transaction::{TransactionMutation, TransactionQuery};

pub struct TransactionService {}

impl TransactionService {
    pub async fn create_transaction<'a>(
        transaction_request: TransactionForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let transaction_id =
            TransactionMutation::create_transaction(transaction_request.into()).await;
        let id = match transaction_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating transaction: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create transaction".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_transaction_by_id<'a>(
        transaction_id: Uuid,
    ) -> Result<TransactionData, AppError> {
        TransactionQuery::get_transaction_by_id(transaction_id).await
    }

    pub async fn get_transactions_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TransactionData>, AppError> {
        TransactionQuery::get_transactions_by_wallet_id(wallet_id, pagination, order).await
    }

    pub async fn get_transactions(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<TransactionData>, AppError> {
        TransactionQuery::get_transactions(pagination, order, filters).await
    }

    pub async fn update_transaction<'a>(
        transaction_id: Uuid,
        transaction_request: TransactionForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result =
            TransactionMutation::update_transaction(transaction_id, transaction_request.into())
                .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating transaction: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update transaction".to_string(),
                ))
            }
        }
    }

    pub async fn delete_transaction<'a>(transaction_id: Uuid) -> Result<bool, AppError> {
        let result = TransactionMutation::delete_transaction(transaction_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting transaction: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete transaction".to_string(),
                ))
            }
        }
    }
}
