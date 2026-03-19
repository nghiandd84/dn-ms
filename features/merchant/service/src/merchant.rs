use tracing::debug;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_merchant_model::merchant::{
    MerchantData, MerchantForCreateRequest, MerchantForUpdateRequest,
};
use features_merchant_repo::merchant::{MerchantMutation, MerchantQuery};

pub struct MerchantService {}

impl MerchantService {
    pub async fn create_merchant(
        merchant_request: MerchantForCreateRequest,
    ) -> Result<String, AppError> {
        let merchant_id = MerchantMutation::create_merchant(merchant_request.into()).await;
        match merchant_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating merchant: {:?}", e);
                Err(AppError::Internal("Failed to create merchant".to_string()))
            }
        }
    }

    pub async fn update_merchant(
        merchant_id: String,
        merchant_request: MerchantForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = MerchantMutation::update_merchant(merchant_id, merchant_request.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating merchant: {:?}", e);
                Err(AppError::Internal("Failed to update merchant".to_string()))
            }
        }
    }

    pub async fn delete_merchant(merchant_id: String) -> Result<bool, AppError> {
        let result = MerchantMutation::delete_merchant(merchant_id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting merchant: {:?}", e);
                Err(AppError::Internal("Failed to delete merchant".to_string()))
            }
        }
    }

    pub async fn get_merchant_by_id(merchant_id: String) -> Result<MerchantData, AppError> {
        MerchantQuery::get_merchant_by_id(merchant_id).await
    }

    pub async fn get_merchants(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<MerchantData>, AppError> {
        MerchantQuery::get_merchants(pagination, order, filters).await
    }
}
