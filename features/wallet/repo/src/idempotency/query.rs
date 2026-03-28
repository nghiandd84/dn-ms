use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::idempotency::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::idempotency::IdempotencyKeyData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct IdempotencyQueryManager;

impl IdempotencyQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct IdempotencyQuery;

impl IdempotencyQuery {
    pub async fn get_idempotency_key_by_id(id: Uuid) -> Result<IdempotencyKeyData, AppError> {
        let model = IdempotencyQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_idempotency_key_by_key(key: &str) -> Result<IdempotencyKeyData, AppError> {
        let filters = vec![FilterEnum::String(FilterParam {
            name: Column::Key.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
            value: Some(key.to_string()),
            raw_value: key.to_string(),
        })];
        let result =
            IdempotencyQueryManager::filter(&Pagination::default(), &Order::default(), &filters)
                .await?;
        if let Some(model) = result.result.into_iter().next() {
            Ok(model.into())
        } else {
            Err(AppError::DuplicateEntry(
                "Idempotency key not found".to_string(),
            ))
        }
    }

    pub async fn get_idempotency_keys(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<IdempotencyKeyData>, AppError> {
        let result = IdempotencyQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
