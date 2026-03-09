use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_core_model::payment_method_limit::{
    PaymentMethodLimitData, PaymentMethodLimitForCreateRequest, PaymentMethodLimitForUpdateRequest,
};
use features_payments_core_repo::payment_method_limit::{
    PaymentMethodLimitMutation, PaymentMethodLimitQuery,
};

pub struct PaymentMethodLimitService {}

impl PaymentMethodLimitService {
    pub async fn create_payment_method_limit<'a>(
        db: &'a DbConn,
        payment_method_limit_request: PaymentMethodLimitForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let payment_method_limit_id = PaymentMethodLimitMutation::create_payment_method_limit(
            db,
            payment_method_limit_request.into(),
        )
        .await;
        let id = match payment_method_limit_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating payment method limit: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create payment method limit".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_payment_method_limit_by_id<'a>(
        db: &'a DbConn,
        payment_method_limit_id: Uuid,
    ) -> Result<PaymentMethodLimitData, AppError> {
        PaymentMethodLimitQuery::get_payment_method_limit_by_id(db, payment_method_limit_id).await
    }

    pub async fn get_payment_method_limits<'a>(
        db: &'a DbConn,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaymentMethodLimitData>, AppError> {
        PaymentMethodLimitQuery::get_payment_method_limits(db, pagination, order, filters).await
    }

    pub async fn update_payment_method_limit(
        db: &DbConn,
        payment_method_limit_id: Uuid,
        payment_method_limit_request: PaymentMethodLimitForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = PaymentMethodLimitMutation::update_payment_method_limit(
            db,
            payment_method_limit_id,
            payment_method_limit_request.into(),
        )
        .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating payment method limit: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update payment method limit".to_string(),
                ))
            }
        }
    }

    pub async fn delete_payment_method_limit(
        db: &DbConn,
        payment_method_limit_id: Uuid,
    ) -> Result<bool, AppError> {
        let result =
            PaymentMethodLimitMutation::delete_payment_method_limit(db, payment_method_limit_id)
                .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting payment method limit: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete payment method limit".to_string(),
                ))
            }
        }
    }
}
