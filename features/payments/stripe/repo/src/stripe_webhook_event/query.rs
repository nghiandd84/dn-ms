use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_stripe_entities::stripe_webhook_event::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_stripe_model::stripe_webhook_event::StripeWebhookEventData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct StripeWebhookEventQueryManager;

impl StripeWebhookEventQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct StripeWebhookEventQuery;

impl StripeWebhookEventQuery {
    pub async fn get_webhook_event_by_id(
        webhook_event_id: Uuid,
    ) -> Result<StripeWebhookEventData, AppError> {
        let model = StripeWebhookEventQueryManager::get_by_id_uuid(webhook_event_id).await?;
        Ok(model.into())
    }

    pub async fn get_webhook_events(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<StripeWebhookEventData>, AppError> {
        let result = StripeWebhookEventQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
