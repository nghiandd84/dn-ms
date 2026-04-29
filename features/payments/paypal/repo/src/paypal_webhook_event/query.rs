use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_paypal_entities::paypal_webhook_event::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_paypal_model::paypal_webhook_event::PaypalWebhookEventData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PaypalWebhookEventQueryManager;

pub struct PaypalWebhookEventQuery;

impl PaypalWebhookEventQuery {
    pub async fn get_webhook_event_by_id(
        webhook_event_id: Uuid,
    ) -> Result<PaypalWebhookEventData, AppError> {
        let model = PaypalWebhookEventQueryManager::get_by_id_uuid(webhook_event_id).await?;
        Ok(model.into())
    }

    pub async fn get_webhook_events(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<PaypalWebhookEventData>, AppError> {
        let result = PaypalWebhookEventQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
