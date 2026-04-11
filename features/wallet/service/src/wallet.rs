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

    /*
    /// Credit wallet balance with optimistic locking
    pub async fn credit_wallet(wallet_id: Uuid, amount: f32) -> Result<WalletData, AppError> {
        if amount <= 0.0 {
            return Err(AppError::BadRequest("Amount must be positive".to_string()));
        }

        // Retry logic for optimistic locking
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        loop {
            attempt += 1;

            // Get current wallet state
            let wallet = Self::get_wallet_by_id(wallet_id).await?;
            let current_version = wallet.version.unwrap_or(1);
            let current_balance = wallet.balance.unwrap_or(0.0);
            let new_balance = current_balance + amount;

            // Create update request with new balance
            let update_request = WalletForUpdateRequest {
                balance: Some(new_balance),
                currency: None,
                is_active: None,
            };

            // For now, use simple update with retry logic
            // In production, you'd implement proper optimistic locking
            match Self::update_wallet(wallet_id, update_request).await {
                Ok(true) => {
                    // Update successful, get updated wallet
                    return Self::get_wallet_by_id(wallet_id).await;
                }
                Err(e) => {
                    if attempt >= MAX_RETRIES {
                        debug!("Failed to debit wallet after {} attempts: {:?}", MAX_RETRIES, e);
                        return Err(AppError::Conflict("Concurrent modification detected. Please retry.".to_string()));
                    }
                    // Small delay before retry
                    tokio::time::sleep(std::time::Duration::from_millis(10 * attempt as u64)).await;
                }
            }
        }
    }

    /// Debit wallet balance with optimistic locking
    pub async fn debit_wallet(wallet_id: Uuid, amount: f32) -> Result<WalletData, AppError> {
        if amount <= 0.0 {
            return Err(AppError::BadRequest("Amount must be positive".to_string()));
        }

        // Retry logic for optimistic locking
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        loop {
            attempt += 1;

            // Get current wallet state
            let wallet = Self::get_wallet_by_id(wallet_id).await?;
            let current_version = wallet.version.unwrap_or(1);
            let current_balance = wallet.balance.unwrap_or(0.0);

            if current_balance < amount {
                return Err(AppError::BadRequest("Insufficient balance".to_string()));
            }

            let new_balance = current_balance - amount;

            // Create update request with new balance
            let update_request = WalletForUpdateRequest {
                balance: Some(new_balance),
                currency: None,
                is_active: None,
            };

            // For now, use simple update with retry logic
            // In production, you'd implement proper optimistic locking
            match Self::update_wallet(wallet_id, update_request).await {
                Ok(true) => {
                    // Update successful, get updated wallet
                    return Self::get_wallet_by_id(wallet_id).await;
                }
                Err(e) => {
                    if attempt >= MAX_RETRIES {
                        debug!("Failed to debit wallet after {} attempts", MAX_RETRIES);
                        return Err(AppError::Conflict("Concurrent modification detected. Please retry.".to_string()));
                    }
                    // Small delay before retry
                    tokio::time::sleep(std::time::Duration::from_millis(10 * attempt as u64)).await;
                }
            }
        }
    }

    /// Transfer money between wallets with proper concurrency control
    pub async fn transfer_between_wallets(
        from_wallet_id: Uuid,
        to_wallet_id: Uuid,
        amount: f32,
    ) -> Result<(WalletData, WalletData), AppError> {
        if amount <= 0.0 {
            return Err(AppError::BadRequest("Amount must be positive".to_string()));
        }

        if from_wallet_id == to_wallet_id {
            return Err(AppError::BadRequest("Cannot transfer to the same wallet".to_string()));
        }

        // For simplicity, we'll do this sequentially with retries
        // In production, you'd want database-level atomic operations

        // First, try to debit from source wallet
        let from_wallet = Self::debit_wallet(from_wallet_id, amount).await?;

        // Then credit to destination wallet
        match Self::credit_wallet(to_wallet_id, amount).await {
            Ok(to_wallet) => Ok((from_wallet, to_wallet)),
            Err(e) => {
                // If credit fails, we need to rollback the debit
                // This is a simplified approach - in production use database transactions
                warn!("Transfer failed, attempting rollback of debit");
                let _ = Self::credit_wallet(from_wallet_id, amount).await; // Rollback
                Err(e)
            }
        }
    }
     */
}
