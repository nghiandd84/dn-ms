use tracing::debug;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_merchant_model::api_key::{
    ApiKeyData, ApiKeyForCreateRequest, ApiKeyForUpdateRequest,
};
use features_merchant_repo::api_key::{ApiKeyMutation, ApiKeyQuery};

pub struct ApiKeyService;

impl ApiKeyService {
    pub async fn create_api_key(api_key_request: ApiKeyForCreateRequest) -> Result<i32, AppError> {
        let api_key_id = ApiKeyMutation::create_api_key(api_key_request.into()).await;
        match api_key_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating API key: {:?}", e);
                Err(AppError::Internal("Failed to create API key".to_string()))
            }
        }
    }

    pub async fn update_api_key(
        api_key_id: i32,
        api_key_request: ApiKeyForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = ApiKeyMutation::update_api_key(api_key_id, api_key_request.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating API key: {:?}", e);
                Err(AppError::Internal("Failed to update API key".to_string()))
            }
        }
    }

    pub async fn delete_api_key(api_key_id: i32) -> Result<bool, AppError> {
        let result = ApiKeyMutation::delete_api_key(api_key_id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting API key: {:?}", e);
                Err(AppError::Internal("Failed to delete API key".to_string()))
            }
        }
    }

    pub async fn get_api_key_by_id(api_key_id: i32) -> Result<ApiKeyData, AppError> {
        ApiKeyQuery::get_api_key_by_id(api_key_id).await
    }

    pub async fn get_api_keys_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        ApiKeyQuery::get_api_keys_by_merchant_id(merchant_id).await
    }

    pub async fn get_api_keys(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        ApiKeyQuery::get_api_keys(pagination, order, filters).await
    }
}
