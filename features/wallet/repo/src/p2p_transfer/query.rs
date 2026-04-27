use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_wallet_entities::p2p_transfer::{ActiveModel, Column, Entity, ModelOptionDto};
use features_wallet_model::p2p_transfer::P2pTransferData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct P2pTransferQueryManager;

impl P2pTransferQueryManager {
}

pub struct P2pTransferQuery;

impl P2pTransferQuery {
    pub async fn get_p2p_transfer_by_id(transfer_id: Uuid) -> Result<P2pTransferData, AppError> {
        let model = P2pTransferQueryManager::get_by_id_uuid(transfer_id).await?;
        Ok(model.into())
    }

    pub async fn get_p2p_transfers(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<P2pTransferData>, AppError> {
        let result = P2pTransferQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_p2p_transfers_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<P2pTransferData>, AppError> {
        let filters = vec![FilterEnum::Uuid(FilterParam {
            name: Column::FromWalletId.to_string(),
            operator: shared_shared_data_core::filter::FilterOperator::Equal,
            value: Some(wallet_id),
            raw_value: wallet_id.to_string(),
        })];
        let result = P2pTransferQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
