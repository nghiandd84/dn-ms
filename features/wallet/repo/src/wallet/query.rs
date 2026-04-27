use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::wallet::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::wallet::WalletData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct WalletQueryManager;

pub struct WalletQuery;

impl WalletQuery {
    pub async fn get_wallet_by_id(wallet_id: Uuid) -> Result<WalletData, AppError> {
        let model = WalletQueryManager::get_by_id_uuid(wallet_id).await?;
        Ok(model.into())
    }

    pub async fn get_wallets<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<WalletData>, AppError> {
        let result = WalletQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_wallet_by_user_id(user_id: Uuid) -> Result<QueryResult<WalletData>, AppError> {
        let filters = vec![FilterEnum::Uuid(FilterParam {
            name: Column::UserId.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
            value: Some(user_id),
            raw_value: user_id.to_string(),
        })];
        let wallet = WalletQueryManager::filter(
            &Pagination::default(),
            &Order::default(),
            &FilterCondition::from(&filters),
        )
        .await?;
        let mapped_result = QueryResult {
            total_page: wallet.total_page,
            result: wallet.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
