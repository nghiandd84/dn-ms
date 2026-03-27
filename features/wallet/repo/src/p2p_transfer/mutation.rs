use features_wallet_entities::p2p_transfer::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, P2pTransferForCreateDto,
    P2pTransferForUpdateDto,
};
use shared_shared_macro::Mutation;

use crate::p2p_transfer::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct P2pTransferMutationManager {}

pub struct P2pTransferMutation;

impl P2pTransferMutation {
    pub fn create_p2p_transfer<'a>(
        data: P2pTransferForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        P2pTransferMutationManager::create_uuid(data.into())
    }

    pub fn update_p2p_transfer<'a>(
        transfer_id: Uuid,
        data: P2pTransferForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        P2pTransferMutationManager::update_by_id_uuid(transfer_id, data.into())
    }

    pub fn delete_p2p_transfer<'a>(
        transfer_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        P2pTransferMutationManager::delete_by_id_uuid(transfer_id)
    }
}
