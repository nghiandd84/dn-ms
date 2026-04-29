use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_paypal_entities::paypal_order::{ActiveModel, Column, Entity, ModelOptionDto};
use features_payments_paypal_model::paypal_order::PaypalOrderData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PaypalOrderQueryManager;

pub struct PaypalOrderQuery;

impl PaypalOrderQuery {
    pub async fn get_order_by_id(order_id: Uuid) -> Result<PaypalOrderData, AppError> {
        let model = PaypalOrderQueryManager::get_by_id_uuid(order_id).await?;
        Ok(model.into())
    }

    pub async fn get_orders(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<PaypalOrderData>, AppError> {
        let result = PaypalOrderQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
