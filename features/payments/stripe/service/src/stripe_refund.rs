use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_payments_stripe_entities::stripe_refund::Column;
use features_payments_stripe_model::stripe_refund::{
    StripeRefundData, StripeRefundForCreateRequest, StripeRefundForUpdateRequest,
};
use features_payments_stripe_repo::stripe_refund::{StripeRefundMutation, StripeRefundQuery};

pub struct StripeRefundService {}

impl StripeRefundService {
    pub async fn create_refund(
        refund_request: StripeRefundForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let refund_id = StripeRefundMutation::create_refund(refund_request.into()).await;
        let id = match refund_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating refund: {:?}", e);
                return Err(AppError::Internal("Failed to create refund".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn bulk_create_refunds(
        refund_requests: Vec<StripeRefundForCreateRequest>,
    ) -> Result<Vec<Uuid>, AppError> {
        let refund_ids = StripeRefundMutation::bulk_create_refunds(
            refund_requests.into_iter().map(|r| r.into()).collect(),
        )
        .await;
        match refund_ids {
            Ok(ids) => Ok(ids),
            Err(e) => {
                debug!("Error bulk creating refunds: {:?}", e);
                Err(AppError::Internal(
                    "Failed to bulk create refunds".to_string(),
                ))
            }
        }
    }

    pub async fn get_refund_by_id(refund_id: Uuid) -> Result<StripeRefundData, AppError> {
        StripeRefundQuery::get_refund_by_id(refund_id).await
    }

    pub async fn get_refunds_by_status(
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeRefundData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        StripeRefundQuery::get_refunds(&pagination, &order, &filters).await
    }

    pub async fn get_refunds(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<StripeRefundData>, AppError> {
        StripeRefundQuery::get_refunds(pagination, order, filters).await
    }

    pub async fn update_refund(
        refund_id: Uuid,
        refund_request: StripeRefundForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = StripeRefundMutation::update_refund(refund_id, refund_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating refund: {:?}", e);
                Err(AppError::Internal("Failed to update refund".to_string()))
            }
        }
    }

    pub async fn delete_refund(refund_id: Uuid) -> Result<bool, AppError> {
        let result = StripeRefundMutation::delete_refund(refund_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting refund: {:?}", e);
                Err(AppError::Internal("Failed to delete refund".to_string()))
            }
        }
    }
}
