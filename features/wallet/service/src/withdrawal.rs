use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_model::withdrawal::{
    WithdrawalData, WithdrawalForCreateRequest, WithdrawalForUpdateRequest,
};
use features_wallet_repo::withdrawal::{WithdrawalMutation, WithdrawalQuery};

pub struct WithdrawalService;

impl WithdrawalService {
    pub async fn create_withdrawal(request: WithdrawalForCreateRequest) -> Result<Uuid, AppError> {
        let withdrawal_id = WithdrawalMutation::create_withdrawal(request.into()).await;
        match withdrawal_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating withdrawal: {:?}", e);
                Err(AppError::Internal(
                    "Failed to create withdrawal".to_string(),
                ))
            }
        }
    }

    pub async fn get_withdrawal_by_id(withdrawal_id: Uuid) -> Result<WithdrawalData, AppError> {
        WithdrawalQuery::get_withdrawal_by_id(withdrawal_id).await
    }

    pub async fn get_withdrawals(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<WithdrawalData>, AppError> {
        WithdrawalQuery::get_withdrawals(pagination, order, filters).await
    }

    pub async fn get_withdrawals_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<WithdrawalData>, AppError> {
        WithdrawalQuery::get_withdrawals_by_wallet_id(wallet_id, pagination, order).await
    }

    pub async fn update_withdrawal(
        withdrawal_id: Uuid,
        request: WithdrawalForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = WithdrawalMutation::update_withdrawal(withdrawal_id, request.into()).await;
        match result {
            Ok(q) => Ok(q),
            Err(e) => {
                debug!("Error updating withdrawal: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update withdrawal".to_string(),
                ))
            }
        }
    }

    pub async fn delete_withdrawal(withdrawal_id: Uuid) -> Result<bool, AppError> {
        let result = WithdrawalMutation::delete_withdrawal(withdrawal_id).await;
        match result {
            Ok(q) => Ok(q),
            Err(e) => {
                debug!("Error deleting withdrawal: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete withdrawal".to_string(),
                ))
            }
        }
    }
}
