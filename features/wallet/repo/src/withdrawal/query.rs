use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::withdrawal::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::withdrawal::WithdrawalData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct WithdrawalQueryManager;

impl WithdrawalQueryManager {
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

pub struct WithdrawalQuery;

impl WithdrawalQuery {
    pub async fn get_withdrawal_by_id(withdrawal_id: Uuid) -> Result<WithdrawalData, AppError> {
        let model = WithdrawalQueryManager::get_by_id_uuid(withdrawal_id).await?;
        Ok(model.into())
    }

    pub async fn get_withdrawals(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<WithdrawalData>, AppError> {
        let result = WithdrawalQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_withdrawals_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<WithdrawalData>, AppError> {
        let filters = vec![FilterEnum::Uuid(FilterParam {
            name: Column::WalletId.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
            value: Some(wallet_id),
            raw_value: wallet_id.to_string(),
        })];
        let result = WithdrawalQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
