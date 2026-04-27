use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_core_entities::payment_attempt::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_core_model::payment_attempt::PaymentAttemptData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PaymentAttemptQueryManager;

pub struct PaymentAttemptQuery;

impl PaymentAttemptQuery {
    pub async fn get_payment_attempt_by_id(
        attempt_id: Uuid,
    ) -> Result<PaymentAttemptData, AppError> {
        let model = PaymentAttemptQueryManager::get_by_id_uuid(attempt_id).await?;
        Ok(model.into())
    }

    pub async fn get_payment_attempts<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<PaymentAttemptData>, AppError> {
        let result = PaymentAttemptQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
