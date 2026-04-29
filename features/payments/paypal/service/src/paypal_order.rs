use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_paypal_model::paypal_order::{
    PaypalOrderData, PaypalOrderForCreateRequest, PaypalOrderForUpdateRequest,
};
use features_payments_paypal_repo::paypal_order::{PaypalOrderMutation, PaypalOrderQuery};

pub struct PaypalOrderService {}

impl PaypalOrderService {
    pub async fn create_order(req: PaypalOrderForCreateRequest) -> Result<Uuid, AppError> {
        PaypalOrderMutation::create_order(req.into())
            .await
            .map_err(|e| {
                debug!("Error creating order: {:?}", e);
                AppError::Internal("Failed to create order".to_string())
            })
    }

    pub async fn get_order_by_id(order_id: Uuid) -> Result<PaypalOrderData, AppError> {
        PaypalOrderQuery::get_order_by_id(order_id).await
    }

    pub async fn get_orders(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaypalOrderData>, AppError> {
        PaypalOrderQuery::get_orders(pagination, order, filters).await
    }

    pub async fn update_order(
        order_id: Uuid,
        req: PaypalOrderForUpdateRequest,
    ) -> Result<bool, AppError> {
        PaypalOrderMutation::update_order(order_id, req.into())
            .await
            .map_err(|e| {
                debug!("Error updating order: {:?}", e);
                AppError::Internal("Failed to update order".to_string())
            })
    }

    pub async fn delete_order(order_id: Uuid) -> Result<bool, AppError> {
        PaypalOrderMutation::delete_order(order_id)
            .await
            .map_err(|e| {
                debug!("Error deleting order: {:?}", e);
                AppError::Internal("Failed to delete order".to_string())
            })
    }
}
