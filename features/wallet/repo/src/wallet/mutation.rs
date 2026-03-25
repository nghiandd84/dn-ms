use shared_shared_macro::Mutation;

use features_wallet_entities::wallet::{
    ActiveModel, WalletForCreateDto, WalletForUpdateDto, Column, Entity, Model, ModelOptionDto,
};

use crate::wallet::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct WalletMutationManager {}

pub struct WalletMutation;

impl WalletMutation {
    pub fn create_wallet<'a>(
        data: WalletForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        WalletMutationManager::create_uuid(data.into())
    }

    pub fn update_wallet<'a>(
        wallet_id: Uuid,
        data: WalletForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WalletMutationManager::update_by_id_uuid(wallet_id, data.into())
    }

    pub fn delete_wallet<'a>(
        wallet_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WalletMutationManager::delete_by_id_uuid(wallet_id)
    }
}
