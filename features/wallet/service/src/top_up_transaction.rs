use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_entities::top_up_transaction::TopUpTransactionForCreateDto;
use features_wallet_model::top_up_transaction::{
    TopUpTransactionData, TopUpTransactionForCreateRequest, TopUpTransactionForUpdateRequest,
};
use features_wallet_repo::top_up_transaction::{TopUpTransactionMutation, TopUpTransactionQuery};

pub struct TopUpTransactionService {}

impl TopUpTransactionService {
    pub async fn create_top_up_transaction<'a>(
        wallet_id: Uuid,
        top_up_request: TopUpTransactionForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let mut create_dto: TopUpTransactionForCreateDto = top_up_request.into();
        create_dto.wallet_id = wallet_id;

        let top_up_id = TopUpTransactionMutation::create_top_up_transaction(create_dto).await;
        let id = match top_up_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating top-up transaction: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create top-up transaction".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_top_up_transaction_by_id<'a>(
        top_up_transaction_id: Uuid,
    ) -> Result<TopUpTransactionData, AppError> {
        TopUpTransactionQuery::get_top_up_transaction_by_id(top_up_transaction_id).await
    }

    pub async fn get_top_up_transactions_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TopUpTransactionData>, AppError> {
        TopUpTransactionQuery::get_top_up_transactions_by_wallet_id(wallet_id, pagination, order)
            .await
    }

    pub async fn get_top_up_transactions(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TopUpTransactionData>, AppError> {
        TopUpTransactionQuery::get_top_up_transactions(pagination, order, filters).await
    }

    pub async fn update_top_up_transaction<'a>(
        top_up_transaction_id: Uuid,
        top_up_request: TopUpTransactionForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = TopUpTransactionMutation::update_top_up_transaction(
            top_up_transaction_id,
            top_up_request.into(),
        )
        .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating top-up transaction: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update top-up transaction".to_string(),
                ))
            }
        }
    }

    pub async fn delete_top_up_transaction<'a>(
        top_up_transaction_id: Uuid,
    ) -> Result<bool, AppError> {
        let result =
            TopUpTransactionMutation::delete_top_up_transaction(top_up_transaction_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting top-up transaction: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete top-up transaction".to_string(),
                ))
            }
        }
    }
}
