use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::top_up_transaction::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::top_up_transaction::TopUpTransactionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TopUpTransactionQueryManager;



pub struct TopUpTransactionQuery;

impl TopUpTransactionQuery {
    pub async fn get_top_up_transaction_by_id(
        top_up_transaction_id: Uuid,
    ) -> Result<TopUpTransactionData, AppError> {
        let model = TopUpTransactionQueryManager::get_by_id_uuid(top_up_transaction_id).await?;
        Ok(model.into())
    }

    pub async fn get_top_up_transactions<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TopUpTransactionData>, AppError> {
        let result = TopUpTransactionQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_top_up_transactions_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TopUpTransactionData>, AppError> {
        let filters = vec![FilterEnum::Uuid(FilterParam {
            name: Column::WalletId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(wallet_id),
            raw_value: wallet_id.to_string(),
        })];
        let result = TopUpTransactionQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
