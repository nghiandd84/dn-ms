use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_core_entities::payment::Column;
use features_payments_core_model::payment::{
    PaymentData, PaymentForCreateRequest, PaymentForUpdateRequest,
};
use features_payments_core_repo::payment::{PaymentMutation, PaymentQuery};

pub struct PaymentService {}

impl PaymentService {
    pub async fn create_payment<'a>(
        payment_request: PaymentForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let payment_id = PaymentMutation::create_payment(payment_request.into()).await;
        let id = match payment_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating payment: {:?}", e);
                return Err(AppError::Internal("Failed to create payment".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_payment_by_id<'a>(payment_id: Uuid) -> Result<PaymentData, AppError> {
        PaymentQuery::get_payment_by_id(payment_id).await
    }

    pub async fn get_payments_by_status(
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        PaymentQuery::get_payments(&pagination, &order, &filters).await
    }

    pub async fn get_payments<'a>(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentData>, AppError> {
        PaymentQuery::get_payments(pagination, order, filters).await
    }

    pub async fn update_payment(
        payment_id: Uuid,
        payment_request: PaymentForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = PaymentMutation::update_payment(payment_id, payment_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating payment: {:?}", e);
                Err(AppError::Internal("Failed to update payment".to_string()))
            }
        }
    }

    pub async fn delete_payment(payment_id: Uuid) -> Result<bool, AppError> {
        let result = PaymentMutation::delete_payment(payment_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting payment: {:?}", e);
                Err(AppError::Internal("Failed to delete payment".to_string()))
            }
        }
    }
}
