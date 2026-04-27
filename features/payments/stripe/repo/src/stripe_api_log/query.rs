use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_stripe_entities::stripe_api_log::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_stripe_model::stripe_api_log::StripeApiLogData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct StripeApiLogQueryManager;

pub struct StripeApiLogQuery;

impl StripeApiLogQuery {
    pub async fn get_api_log_by_id(api_log_id: Uuid) -> Result<StripeApiLogData, AppError> {
        let model = StripeApiLogQueryManager::get_by_id_uuid(api_log_id).await?;
        Ok(model.into())
    }

    pub async fn get_api_logs(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<StripeApiLogData>, AppError> {
        let result = StripeApiLogQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
