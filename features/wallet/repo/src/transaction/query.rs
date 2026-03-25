use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::transaction::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::transaction::TransactionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TransactionQueryManager;

impl TransactionQueryManager {
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

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_transaction_by_id(transaction_id: Uuid) -> Result<TransactionData, AppError> {
        let model = TransactionQueryManager::get_by_id_uuid(transaction_id).await?;
        Ok(model.into())
    }

    pub async fn get_transactions<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TransactionData>, AppError> {
        let result = TransactionQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_transactions_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TransactionData>, AppError> {
        let filters = vec![FilterEnum::Uuid(FilterParam {
            name: Column::WalletId.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
            value: Some(wallet_id),
            raw_value: wallet_id.to_string(),
        })];
        let result = TransactionQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
