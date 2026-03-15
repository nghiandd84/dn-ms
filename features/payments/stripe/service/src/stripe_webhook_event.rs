use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_stripe_entities::stripe_webhook_event::Column;
use features_payments_stripe_model::stripe_webhook_event::{StripeWebhookEventData, StripeWebhookEventForCreateRequest, StripeWebhookEventForUpdateRequest};
use features_payments_stripe_repo::stripe_webhook_event::{StripeWebhookEventMutation, StripeWebhookEventQuery};

pub struct StripeWebhookEventService {}

impl StripeWebhookEventService {
    pub async fn create_webhook_event(webhook_event_request: StripeWebhookEventForCreateRequest) -> Result<Uuid, AppError> {
        let webhook_event_id = StripeWebhookEventMutation::create_webhook_event(webhook_event_request.into()).await;
        let id = match webhook_event_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating webhook event: {:?}", e);
                return Err(AppError::Internal("Failed to create webhook event".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn bulk_create_webhook_events(
        webhook_event_requests: Vec<StripeWebhookEventForCreateRequest>,
    ) -> Result<Vec<Uuid>, AppError> {
        let webhook_event_ids =
            StripeWebhookEventMutation::bulk_create_webhook_events(webhook_event_requests.into_iter().map(|r| r.into()).collect())
                .await;
        match webhook_event_ids {
            Ok(ids) => Ok(ids),
            Err(e) => {
                debug!("Error bulk creating webhook events: {:?}", e);
                Err(AppError::Internal(
                    "Failed to bulk create webhook events".to_string(),
                ))
            }
        }
    }

    pub async fn get_webhook_event_by_id(webhook_event_id: Uuid) -> Result<StripeWebhookEventData, AppError> {
        StripeWebhookEventQuery::get_webhook_event_by_id(webhook_event_id).await
    }

    pub async fn get_webhook_events_by_processed(
        processed: bool,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeWebhookEventData>, AppError> {
        let processed_column = Column::Processed.to_string();
        let param: FilterParam<bool> = FilterParam {
            name: processed_column,
            operator: FilterOperator::Equal,
            value: Some(processed),
            raw_value: processed.to_string(),
        };
        let processed_filter = FilterEnum::Bool(param);
        let filters: Vec<FilterEnum> = vec![processed_filter];
        StripeWebhookEventQuery::get_webhook_events(&pagination, &order, &filters).await
    }

    pub async fn get_webhook_events(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeWebhookEventData>, AppError> {
        StripeWebhookEventQuery::get_webhook_events(pagination, order, filters).await
    }

    pub async fn update_webhook_event(
        webhook_event_id: Uuid,
        webhook_event_request: StripeWebhookEventForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = StripeWebhookEventMutation::update_webhook_event(webhook_event_id, webhook_event_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating webhook event: {:?}", e);
                Err(AppError::Internal("Failed to update webhook event".to_string()))
            }
        }
    }

    pub async fn delete_webhook_event(webhook_event_id: Uuid) -> Result<bool, AppError> {
        let result = StripeWebhookEventMutation::delete_webhook_event(webhook_event_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting webhook event: {:?}", e);
                Err(AppError::Internal("Failed to delete webhook event".to_string()))
            }
        }
    }
}