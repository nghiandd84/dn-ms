use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_model::p2p_transfer::{P2pTransferData, P2pTransferForCreateRequest, P2pTransferForUpdateRequest};
use features_wallet_repo::p2p_transfer::{P2pTransferMutation, P2pTransferQuery};

pub struct P2pTransferService;

impl P2pTransferService {
    pub async fn create_p2p_transfer(
        request: P2pTransferForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let transfer_id = P2pTransferMutation::create_p2p_transfer(request.into()).await;
        match transfer_id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating p2p transfer: {:?}", e);
                Err(AppError::Internal("Failed to create p2p transfer".to_string()))
            }
        }
    }

    pub async fn get_p2p_transfer_by_id(transfer_id: Uuid) -> Result<P2pTransferData, AppError> {
        P2pTransferQuery::get_p2p_transfer_by_id(transfer_id).await
    }

    pub async fn get_p2p_transfers(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<P2pTransferData>, AppError> {
        P2pTransferQuery::get_p2p_transfers(pagination, order, filters).await
    }

    pub async fn get_p2p_transfers_by_wallet_id(
        wallet_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<P2pTransferData>, AppError> {
        P2pTransferQuery::get_p2p_transfers_by_wallet_id(wallet_id, pagination, order).await
    }

    pub async fn update_p2p_transfer(
        transfer_id: Uuid,
        request: P2pTransferForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = P2pTransferMutation::update_p2p_transfer(transfer_id, request.into()).await;
        match result {
            Ok(q) => Ok(q),
            Err(e) => {
                debug!("Error updating p2p transfer: {:?}", e);
                Err(AppError::Internal("Failed to update p2p transfer".to_string()))
            }
        }
    }

    pub async fn delete_p2p_transfer(transfer_id: Uuid) -> Result<bool, AppError> {
        let result = P2pTransferMutation::delete_p2p_transfer(transfer_id).await;
        match result {
            Ok(q) => Ok(q),
            Err(e) => {
                debug!("Error deleting p2p transfer: {:?}", e);
                Err(AppError::Internal("Failed to delete p2p transfer".to_string()))
            }
        }
    }
}
