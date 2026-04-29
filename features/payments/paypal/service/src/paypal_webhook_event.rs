use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_paypal_model::paypal_webhook_event::{
    PaypalWebhookEventData, PaypalWebhookEventForCreateRequest,
    PaypalWebhookEventForUpdateRequest,
};
use features_payments_paypal_repo::paypal_webhook_event::{
    PaypalWebhookEventMutation, PaypalWebhookEventQuery,
};

pub struct PaypalWebhookEventService {}

impl PaypalWebhookEventService {
    pub async fn create_webhook_event(
        req: PaypalWebhookEventForCreateRequest,
    ) -> Result<Uuid, AppError> {
        PaypalWebhookEventMutation::create_webhook_event(req.into())
            .await
            .map_err(|e| {
                debug!("Error creating webhook event: {:?}", e);
                AppError::Internal("Failed to create webhook event".to_string())
            })
    }

    pub async fn get_webhook_event_by_id(
        webhook_event_id: Uuid,
    ) -> Result<PaypalWebhookEventData, AppError> {
        PaypalWebhookEventQuery::get_webhook_event_by_id(webhook_event_id).await
    }

    pub async fn get_webhook_events(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaypalWebhookEventData>, AppError> {
        PaypalWebhookEventQuery::get_webhook_events(pagination, order, filters).await
    }

    pub async fn update_webhook_event(
        webhook_event_id: Uuid,
        req: PaypalWebhookEventForUpdateRequest,
    ) -> Result<bool, AppError> {
        PaypalWebhookEventMutation::update_webhook_event(webhook_event_id, req.into())
            .await
            .map_err(|e| {
                debug!("Error updating webhook event: {:?}", e);
                AppError::Internal("Failed to update webhook event".to_string())
            })
    }

    pub async fn delete_webhook_event(webhook_event_id: Uuid) -> Result<bool, AppError> {
        PaypalWebhookEventMutation::delete_webhook_event(webhook_event_id)
            .await
            .map_err(|e| {
                debug!("Error deleting webhook event: {:?}", e);
                AppError::Internal("Failed to delete webhook event".to_string())
            })
    }
}
