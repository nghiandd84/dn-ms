use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_url_shortener_entities::api_key::{ActiveModel, Column, Entity, ModelOptionDto};
use features_url_shortener_model::api_key::ApiKeyData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ApiKeyQueryManager;

pub struct ApiKeyQuery;

impl ApiKeyQuery {
    pub async fn get_by_key_hash(key_hash: &str) -> Result<ApiKeyData, AppError> {
        let hash_param: FilterParam<String> = FilterParam {
            name: Column::KeyHash.to_string(),
            operator: FilterOperator::Equal,
            value: Some(key_hash.to_string()),
            raw_value: key_hash.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(hash_param)].into();
        let pagination = Pagination::default();
        let order = Order::default();

        let result = ApiKeyQueryManager::filter(&pagination, &order, &filters).await?;
        let item = result
            .result
            .into_iter()
            .next()
            .ok_or(AppError::EntityNotFound {
                entity: "api_key".to_string(),
            })?;
        Ok(item.into())
    }

    pub async fn list_by_user_id(
        user_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        let filters: FilterCondition = vec![FilterEnum::Uuid(FilterParam {
            name: Column::UserId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(*user_id),
            raw_value: user_id.to_string(),
        })]
        .into();

        let result = ApiKeyQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
