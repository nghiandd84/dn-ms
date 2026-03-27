use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "p2p_transfers")]
#[dto(
    name(P2pTransferForCreate),
    columns(from_wallet_id, to_wallet_id, amount, status, completed_at)
)]
#[dto(name(P2pTransferForUpdate), columns(status, completed_at), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub from_wallet_id: Uuid,
    pub to_wallet_id: Uuid,
    pub amount: f32,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub completed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::wallet::Entity",
        from = "Column::FromWalletId",
        to = "super::wallet::Column::Id"
    )]
    FromWallet,
    #[sea_orm(
        belongs_to = "super::wallet::Entity",
        from = "Column::ToWalletId",
        to = "super::wallet::Column::Id"
    )]
    ToWallet,
}

impl Related<super::wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FromWallet.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        if insert {
            self.created_at = ActiveValue::Set(current_time);
        }
        self.updated_at = ActiveValue::Set(current_time);
        Ok(self)
    }
}
