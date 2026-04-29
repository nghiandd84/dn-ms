use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_paypal_model::paypal_api_log::{
    PaypalApiLogData, PaypalApiLogForCreateRequest, PaypalApiLogForUpdateRequest,
};
use features_payments_paypal_repo::paypal_api_log::{PaypalApiLogMutation, PaypalApiLogQuery};

pub struct PaypalApiLogService {}

impl PaypalApiLogService {
    pub async fn create_api_log(req: PaypalApiLogForCreateRequest) -> Result<Uuid, AppError> {
        PaypalApiLogMutation::create_api_log(req.into())
            .await
            .map_err(|e| {
                debug!("Error creating api log: {:?}", e);
                AppError::Internal("Failed to create api log".to_string())
            })
    }

    pub async fn get_api_log_by_id(api_log_id: Uuid) -> Result<PaypalApiLogData, AppError> {
        PaypalApiLogQuery::get_api_log_by_id(api_log_id).await
    }

    pub async fn get_api_logs(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<PaypalApiLogData>, AppError> {
        PaypalApiLogQuery::get_api_logs(pagination, order, filters).await
    }

    pub async fn update_api_log(
        api_log_id: Uuid,
        req: PaypalApiLogForUpdateRequest,
    ) -> Result<bool, AppError> {
        PaypalApiLogMutation::update_api_log(api_log_id, req.into())
            .await
            .map_err(|e| {
                debug!("Error updating api log: {:?}", e);
                AppError::Internal("Failed to update api log".to_string())
            })
    }

    pub async fn delete_api_log(api_log_id: Uuid) -> Result<bool, AppError> {
        PaypalApiLogMutation::delete_api_log(api_log_id)
            .await
            .map_err(|e| {
                debug!("Error deleting api log: {:?}", e);
                AppError::Internal("Failed to delete api log".to_string())
            })
    }
}
