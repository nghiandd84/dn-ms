use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_core_entities::payment_method::Column;
use features_payments_core_model::payment_method::{
    PaymentMethodData, PaymentMethodForCreateRequest, PaymentMethodForUpdateRequest,
};
use features_payments_core_repo::payment_method::{PaymentMethodMutation, PaymentMethodQuery};

pub struct PaymentMethodService {}

impl PaymentMethodService {
    pub async fn create_payment_method<'a>(
        payment_method_request: PaymentMethodForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let payment_method_id =
            PaymentMethodMutation::create_payment_method(payment_method_request.into()).await;
        let id = match payment_method_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating payment method: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create payment method".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_payment_method_by_id<'a>(
        payment_method_id: Uuid,
    ) -> Result<PaymentMethodData, AppError> {
        PaymentMethodQuery::get_payment_method_by_id(payment_method_id).await
    }

    pub async fn get_payment_methods_by_active(
        is_active: bool,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentMethodData>, AppError> {
        let active_column = Column::IsActive.to_string();
        let param: FilterParam<bool> = FilterParam {
            name: active_column,
            operator: FilterOperator::Equal,
            value: Some(is_active),
            raw_value: is_active.to_string(),
        };
        let active_filter = FilterEnum::Bool(param);
        let filters: Vec<FilterEnum> = vec![active_filter];
        PaymentMethodQuery::get_payment_methods(&pagination, &order, &filters).await
    }

    pub async fn get_payment_methods<'a>(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentMethodData>, AppError> {
        PaymentMethodQuery::get_payment_methods(pagination, order, filters).await
    }

    pub async fn update_payment_method(
        payment_method_id: Uuid,
        payment_method_request: PaymentMethodForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = PaymentMethodMutation::update_payment_method(
            payment_method_id,
            payment_method_request.into(),
        )
        .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating payment method: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update payment method".to_string(),
                ))
            }
        }
    }

    pub async fn delete_payment_method(payment_method_id: Uuid) -> Result<bool, AppError> {
        let result = PaymentMethodMutation::delete_payment_method(payment_method_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting payment method: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete payment method".to_string(),
                ))
            }
        }
    }
}
