use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;
use uuid::Uuid;

use features_fee_entities::fee_configuration::{ActiveModel, Column, Entity, ModelOptionDto};
use features_fee_model::fee_configuration::FeeConfigurationData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct FeeConfigurationQueryManager;



pub struct FeeConfigurationQuery;

impl FeeConfigurationQuery {
    pub async fn get_fee_configuration_by_id(
        fee_configuration_id: Uuid,
    ) -> Result<FeeConfigurationData, AppError> {
        let model = FeeConfigurationQueryManager::get_by_id_uuid(fee_configuration_id).await?;
        Ok(model.into())
    }

    pub async fn get_fee_configurations_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<FeeConfigurationData>, AppError> {
        let merchant_id_filter = FilterEnum::String(shared_shared_data_core::filter::FilterParam {
            name: Column::MerchantId.to_string(),
            value: Some(merchant_id.clone()),
            raw_value: merchant_id.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
        });
        let filters = vec![merchant_id_filter];
        let result = FeeConfigurationQueryManager::filter(
            &Pagination::default(),
            &Order::default(),
            &filters,
        )
        .await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|f| f.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_fee_configurations<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<FeeConfigurationData>, AppError> {
        let result = FeeConfigurationQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|f| f.into()).collect(),
        };
        Ok(mapped_result)
    }
}
