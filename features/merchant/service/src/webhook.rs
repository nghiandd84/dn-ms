use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_merchant_model::webhook::{
    WebhookData, WebhookForCreateRequest, WebhookForUpdateRequest,
};
use features_merchant_repo::webhook::{WebhookMutation, WebhookQuery};

pub struct WebhookService;

impl WebhookService {
    pub async fn create_webhook(webhook_request: WebhookForCreateRequest) -> Result<Uuid, AppError> {
        // TODO: Add URL validation (https only, no private IPs, etc.)
        let webhook_id = WebhookMutation::create_webhook(webhook_request.into()).await;
        match webhook_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating webhook: {:?}", e);
                Err(AppError::Internal("Failed to create webhook".to_string()))
            }
        }
    }

    pub async fn update_webhook(
        webhook_id: Uuid,
        webhook_request: WebhookForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = WebhookMutation::update_webhook(webhook_id, webhook_request.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating webhook: {:?}", e);
                Err(AppError::Internal("Failed to update webhook".to_string()))
            }
        }
    }

    pub async fn delete_webhook(webhook_id: Uuid) -> Result<bool, AppError> {
        let result = WebhookMutation::delete_webhook(webhook_id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting webhook: {:?}", e);
                Err(AppError::Internal("Failed to delete webhook".to_string()))
            }
        }
    }

    pub async fn get_webhook_by_id(webhook_id: Uuid) -> Result<WebhookData, AppError> {
        WebhookQuery::get_webhook_by_id(webhook_id).await
    }

    pub async fn get_webhooks_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<WebhookData>, AppError> {
        WebhookQuery::get_webhooks_by_merchant_id(merchant_id).await
    }

    pub async fn get_webhooks(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<WebhookData>, AppError> {
        WebhookQuery::get_webhooks(pagination, order, filters).await
    }
}