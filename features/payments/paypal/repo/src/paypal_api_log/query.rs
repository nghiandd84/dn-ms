use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_paypal_entities::paypal_api_log::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_paypal_model::paypal_api_log::PaypalApiLogData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PaypalApiLogQueryManager;

pub struct PaypalApiLogQuery;

impl PaypalApiLogQuery {
    pub async fn get_api_log_by_id(api_log_id: Uuid) -> Result<PaypalApiLogData, AppError> {
        let model = PaypalApiLogQueryManager::get_by_id_uuid(api_log_id).await?;
        Ok(model.into())
    }

    pub async fn get_api_logs(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<PaypalApiLogData>, AppError> {
        let result = PaypalApiLogQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
