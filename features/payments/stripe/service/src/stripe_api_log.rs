use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_stripe_entities::stripe_api_log::Column;
use features_payments_stripe_model::stripe_api_log::{StripeApiLogData, StripeApiLogForCreateRequest, StripeApiLogForUpdateRequest};
use features_payments_stripe_repo::stripe_api_log::{StripeApiLogMutation, StripeApiLogQuery};

pub struct StripeApiLogService {}

impl StripeApiLogService {
    pub async fn create_api_log(api_log_request: StripeApiLogForCreateRequest) -> Result<Uuid, AppError> {
        let api_log_id = StripeApiLogMutation::create_api_log(api_log_request.into()).await;
        let id = match api_log_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating api log: {:?}", e);
                return Err(AppError::Internal("Failed to create api log".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn bulk_create_api_logs(
        api_log_requests: Vec<StripeApiLogForCreateRequest>,
    ) -> Result<Vec<Uuid>, AppError> {
        let api_log_ids =
            StripeApiLogMutation::bulk_create_api_logs(api_log_requests.into_iter().map(|r| r.into()).collect())
                .await;
        match api_log_ids {
            Ok(ids) => Ok(ids),
            Err(e) => {
                debug!("Error bulk creating api logs: {:?}", e);
                Err(AppError::Internal(
                    "Failed to bulk create api logs".to_string(),
                ))
            }
        }
    }

    pub async fn get_api_log_by_id(api_log_id: Uuid) -> Result<StripeApiLogData, AppError> {
        StripeApiLogQuery::get_api_log_by_id(api_log_id).await
    }

    pub async fn get_api_logs_by_method(
        method: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeApiLogData>, AppError> {
        let method_column = Column::Method.to_string();
        let param: FilterParam<String> = FilterParam {
            name: method_column,
            operator: FilterOperator::Equal,
            value: Some(method.to_string()),
            raw_value: method.to_string(),
        };
        let method_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![method_filter];
        StripeApiLogQuery::get_api_logs(&pagination, &order, &filters).await
    }

    pub async fn get_api_logs(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeApiLogData>, AppError> {
        StripeApiLogQuery::get_api_logs(pagination, order, filters).await
    }

    pub async fn update_api_log(
        api_log_id: Uuid,
        api_log_request: StripeApiLogForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = StripeApiLogMutation::update_api_log(api_log_id, api_log_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating api log: {:?}", e);
                Err(AppError::Internal("Failed to update api log".to_string()))
            }
        }
    }

    pub async fn delete_api_log(api_log_id: Uuid) -> Result<bool, AppError> {
        let result = StripeApiLogMutation::delete_api_log(api_log_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting api log: {:?}", e);
                Err(AppError::Internal("Failed to delete api log".to_string()))
            }
        }
    }
}