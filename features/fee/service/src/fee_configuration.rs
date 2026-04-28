use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_fee_model::fee_configuration::{
    FeeConfigurationData, FeeConfigurationForCreateRequest, FeeConfigurationForUpdateRequest,
};
use features_fee_repo::fee_configuration::{FeeConfigurationMutation, FeeConfigurationQuery};

pub struct FeeConfigurationService;

impl FeeConfigurationService {
    pub async fn create_fee_configuration(
        fee_configuration_request: FeeConfigurationForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let fee_configuration_id =
            FeeConfigurationMutation::create_fee_configuration(fee_configuration_request.into())
                .await;
        match fee_configuration_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating fee configuration: {:?}", e);
                Err(AppError::Internal(
                    "Failed to create fee configuration".to_string(),
                ))
            }
        }
    }

    pub async fn update_fee_configuration(
        fee_configuration_id: Uuid,
        fee_configuration_request: FeeConfigurationForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = FeeConfigurationMutation::update_fee_configuration(
            fee_configuration_id,
            fee_configuration_request.into(),
        )
        .await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating fee configuration: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update fee configuration".to_string(),
                ))
            }
        }
    }

    pub async fn delete_fee_configuration(fee_configuration_id: Uuid) -> Result<bool, AppError> {
        let result = FeeConfigurationMutation::delete_fee_configuration(fee_configuration_id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting fee configuration: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete fee configuration".to_string(),
                ))
            }
        }
    }

    pub async fn get_fee_configuration_by_id(
        fee_configuration_id: Uuid,
    ) -> Result<FeeConfigurationData, AppError> {
        FeeConfigurationQuery::get_fee_configuration_by_id(fee_configuration_id).await
    }

    pub async fn get_fee_configurations_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<FeeConfigurationData>, AppError> {
        FeeConfigurationQuery::get_fee_configurations_by_merchant_id(merchant_id).await
    }

    pub async fn get_fee_configurations(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<FeeConfigurationData>, AppError> {
        FeeConfigurationQuery::get_fee_configurations(pagination, order, filters).await
    }
}
