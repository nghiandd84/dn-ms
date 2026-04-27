use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_stripe_entities::stripe_refund::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_stripe_model::stripe_refund::StripeRefundData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct StripeRefundQueryManager;

pub struct StripeRefundQuery;

impl StripeRefundQuery {
    pub async fn get_refund_by_id(refund_id: Uuid) -> Result<StripeRefundData, AppError> {
        let model = StripeRefundQueryManager::get_by_id_uuid(refund_id).await?;
        Ok(model.into())
    }

    pub async fn get_refunds(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<StripeRefundData>, AppError> {
        let result = StripeRefundQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
