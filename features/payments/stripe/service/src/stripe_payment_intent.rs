use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_stripe_entities::stripe_payment_intent::Column;
use features_payments_stripe_model::stripe_payment_intent::{StripePaymentIntentData, StripePaymentIntentForCreateRequest, StripePaymentIntentForUpdateRequest};
use features_payments_stripe_repo::stripe_payment_intent::{StripePaymentIntentMutation, StripePaymentIntentQuery};

pub struct StripePaymentIntentService {}

impl StripePaymentIntentService {
    pub async fn create_payment_intent(payment_intent_request: StripePaymentIntentForCreateRequest) -> Result<Uuid, AppError> {
        let payment_intent_id = StripePaymentIntentMutation::create_payment_intent(payment_intent_request.into()).await;
        let id = match payment_intent_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating payment intent: {:?}", e);
                return Err(AppError::Internal("Failed to create payment intent".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn bulk_create_payment_intents(
        payment_intent_requests: Vec<StripePaymentIntentForCreateRequest>,
    ) -> Result<Vec<Uuid>, AppError> {
        let payment_intent_ids =
            StripePaymentIntentMutation::bulk_create_payment_intents(payment_intent_requests.into_iter().map(|r| r.into()).collect())
                .await;
        match payment_intent_ids {
            Ok(ids) => Ok(ids),
            Err(e) => {
                debug!("Error bulk creating payment intents: {:?}", e);
                Err(AppError::Internal(
                    "Failed to bulk create payment intents".to_string(),
                ))
            }
        }
    }

    pub async fn get_payment_intent_by_id(payment_intent_id: Uuid) -> Result<StripePaymentIntentData, AppError> {
        StripePaymentIntentQuery::get_payment_intent_by_id(payment_intent_id).await
    }

    pub async fn get_payment_intents_by_status(
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripePaymentIntentData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        StripePaymentIntentQuery::get_payment_intents(&pagination, &order, &filters).await
    }

    pub async fn get_payment_intents(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripePaymentIntentData>, AppError> {
        StripePaymentIntentQuery::get_payment_intents(pagination, order, filters).await
    }

    pub async fn update_payment_intent(
        payment_intent_id: Uuid,
        payment_intent_request: StripePaymentIntentForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = StripePaymentIntentMutation::update_payment_intent(payment_intent_id, payment_intent_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating payment intent: {:?}", e);
                Err(AppError::Internal("Failed to update payment intent".to_string()))
            }
        }
    }

    pub async fn delete_payment_intent(payment_intent_id: Uuid) -> Result<bool, AppError> {
        let result = StripePaymentIntentMutation::delete_payment_intent(payment_intent_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting payment intent: {:?}", e);
                Err(AppError::Internal("Failed to delete payment intent".to_string()))
            }
        }
    }
}