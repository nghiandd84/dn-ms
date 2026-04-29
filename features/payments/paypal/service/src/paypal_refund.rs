use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_paypal_model::paypal_refund::{
    PaypalRefundData, PaypalRefundForCreateRequest, PaypalRefundForUpdateRequest,
};
use features_payments_paypal_repo::paypal_refund::{PaypalRefundMutation, PaypalRefundQuery};

pub struct PaypalRefundService {}

impl PaypalRefundService {
    pub async fn create_refund(req: PaypalRefundForCreateRequest) -> Result<Uuid, AppError> {
        PaypalRefundMutation::create_refund(req.into())
            .await
            .map_err(|e| {
                debug!("Error creating refund: {:?}", e);
                AppError::Internal("Failed to create refund".to_string())
            })
    }

    pub async fn get_refund_by_id(refund_id: Uuid) -> Result<PaypalRefundData, AppError> {
        PaypalRefundQuery::get_refund_by_id(refund_id).await
    }

    pub async fn get_refunds(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaypalRefundData>, AppError> {
        PaypalRefundQuery::get_refunds(pagination, order, filters).await
    }

    pub async fn update_refund(
        refund_id: Uuid,
        req: PaypalRefundForUpdateRequest,
    ) -> Result<bool, AppError> {
        PaypalRefundMutation::update_refund(refund_id, req.into())
            .await
            .map_err(|e| {
                debug!("Error updating refund: {:?}", e);
                AppError::Internal("Failed to update refund".to_string())
            })
    }

    pub async fn delete_refund(refund_id: Uuid) -> Result<bool, AppError> {
        PaypalRefundMutation::delete_refund(refund_id)
            .await
            .map_err(|e| {
                debug!("Error deleting refund: {:?}", e);
                AppError::Internal("Failed to delete refund".to_string())
            })
    }
}
