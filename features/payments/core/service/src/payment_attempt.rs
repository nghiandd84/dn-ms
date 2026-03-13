use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_core_entities::payment_attempt::Column;
use features_payments_core_model::payment_attempt::{
    PaymentAttemptData, PaymentAttemptForCreateRequest, PaymentAttemptForUpdateRequest,
};
use features_payments_core_repo::payment_attempt::{PaymentAttemptMutation, PaymentAttemptQuery};

pub struct PaymentAttemptService {}

impl PaymentAttemptService {
    pub async fn create_payment_attempt<'a>(
        payment_attempt_request: PaymentAttemptForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let payment_attempt_id =
            PaymentAttemptMutation::create_payment_attempt(payment_attempt_request.into()).await;
        let id = match payment_attempt_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating payment attempt: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create payment attempt".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_payment_attempt_by_id<'a>(
        payment_attempt_id: Uuid,
    ) -> Result<PaymentAttemptData, AppError> {
        PaymentAttemptQuery::get_payment_attempt_by_id(payment_attempt_id).await
    }

    pub async fn get_payment_attempts_by_success(
        success: bool,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentAttemptData>, AppError> {
        let success_column = Column::Success.to_string();
        let param: FilterParam<bool> = FilterParam {
            name: success_column,
            operator: FilterOperator::Equal,
            value: Some(success),
            raw_value: success.to_string(),
        };
        let success_filter = FilterEnum::Bool(param);
        let filters: Vec<FilterEnum> = vec![success_filter];
        PaymentAttemptQuery::get_payment_attempts(&pagination, &order, &filters).await
    }

    pub async fn get_payment_attempts<'a>(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentAttemptData>, AppError> {
        PaymentAttemptQuery::get_payment_attempts(pagination, order, filters).await
    }

    pub async fn update_payment_attempt(
        payment_attempt_id: Uuid,
        payment_attempt_request: PaymentAttemptForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = PaymentAttemptMutation::update_payment_attempt(
            payment_attempt_id,
            payment_attempt_request.into(),
        )
        .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating payment attempt: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update payment attempt".to_string(),
                ))
            }
        }
    }

    pub async fn delete_payment_attempt(payment_attempt_id: Uuid) -> Result<bool, AppError> {
        let result = PaymentAttemptMutation::delete_payment_attempt(payment_attempt_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting payment attempt: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete payment attempt".to_string(),
                ))
            }
        }
    }
}
