use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;
use uuid::Uuid;

use features_merchant_entities::webhook::{ActiveModel, Column, Entity, ModelOptionDto};
use features_merchant_model::webhook::WebhookData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct WebhookQueryManager;

impl WebhookQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> sea_orm::Condition {
        let mut condition = sea_orm::Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct WebhookQuery;

impl WebhookQuery {
    pub async fn get_webhook_by_id(webhook_id: Uuid) -> Result<WebhookData, AppError> {
        let model = WebhookQueryManager::get_by_id_uuid(webhook_id).await?;
        Ok(model.into())
    }

    pub async fn get_webhooks_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<WebhookData>, AppError> {
        let merchant_id_filter = FilterEnum::String(shared_shared_data_core::filter::FilterParam {
            name: Column::MerchantId.to_string(),
            value: Some(merchant_id.clone()),
            raw_value: merchant_id.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
        });
        let filters = vec![merchant_id_filter];
        let result =
            WebhookQueryManager::filter(&Pagination::default(), &Order::default(), &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|w| w.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_webhooks<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<WebhookData>, AppError> {
        let result = WebhookQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|w| w.into()).collect(),
        };
        Ok(mapped_result)
    }
}