use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_model::wallet::{WalletData, WalletForCreateRequest, WalletForUpdateRequest};
use features_wallet_repo::wallet::{WalletMutation, WalletQuery};

pub struct WalletService {}

impl WalletService {
    pub async fn create_wallet<'a>(
        wallet_request: WalletForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let create_dto = wallet_request.into();

        let wallet_id = WalletMutation::create_wallet(create_dto).await;
        let id = match wallet_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating wallet: {:?}", e);
                return Err(AppError::Internal("Failed to create wallet".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_wallet_by_id<'a>(wallet_id: Uuid) -> Result<WalletData, AppError> {
        WalletQuery::get_wallet_by_id(wallet_id).await
    }

    pub async fn get_wallet_by_user_id<'a>(
        user_id: Uuid,
    ) -> Result<QueryResult<WalletData>, AppError> {
        WalletQuery::get_wallet_by_user_id(user_id).await
    }

    pub async fn get_wallets(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<WalletData>, AppError> {
        WalletQuery::get_wallets(pagination, order, filters).await
    }

    pub async fn update_wallet<'a>(
        wallet_id: Uuid,
        wallet_request: WalletForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = WalletMutation::update_wallet(wallet_id, wallet_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating wallet: {:?}", e);
                Err(AppError::Internal("Failed to update wallet".to_string()))
            }
        }
    }

    pub async fn delete_wallet<'a>(wallet_id: Uuid) -> Result<bool, AppError> {
        let result = WalletMutation::delete_wallet(wallet_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting wallet: {:?}", e);
                Err(AppError::Internal("Failed to delete wallet".to_string()))
            }
        }
    }
}
