use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_stripe_entities::stripe_payment_intent::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_payments_stripe_model::stripe_payment_intent::StripePaymentIntentData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct StripePaymentIntentQueryManager;

impl StripePaymentIntentQueryManager {
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

pub struct StripePaymentIntentQuery;

impl StripePaymentIntentQuery {
    pub async fn get_payment_intent_by_id(
        payment_intent_id: Uuid,
    ) -> Result<StripePaymentIntentData, AppError> {
        let model = StripePaymentIntentQueryManager::get_by_id_uuid(payment_intent_id).await?;
        Ok(model.into())
    }

    pub async fn get_payment_intents(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<StripePaymentIntentData>, AppError> {
        let result = StripePaymentIntentQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
