use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_merchant_entities::api_key::{ActiveModel, Column, Entity, ModelOptionDto};
use features_merchant_model::api_key::ApiKeyData;

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
struct ApiKeyQueryManager;

impl ApiKeyQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> sea_orm::Condition {
        let mut condition = sea_orm::Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct ApiKeyQuery;

impl ApiKeyQuery {
    pub async fn get_api_key_by_id(api_key_id: i32) -> Result<ApiKeyData, AppError> {
        let model = ApiKeyQueryManager::get_by_id_i32(api_key_id).await?;
        Ok(model.into())
    }

    pub async fn get_api_keys_by_merchant_id(
        merchant_id: String,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        let merchant_id_filter = FilterEnum::String(FilterParam {
            name: Column::MerchantId.to_string(),
            value: Some(merchant_id.clone()),
            raw_value: merchant_id.to_string(),
            operator: FilterOperator::Equal,
        });
        let filters = vec![merchant_id_filter];
        let result =
            ApiKeyQueryManager::filter(&Pagination::default(), &Order::default(), &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|a| a.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_api_keys<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        let result = ApiKeyQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|a| a.into()).collect(),
        };
        Ok(mapped_result)
    }
}
