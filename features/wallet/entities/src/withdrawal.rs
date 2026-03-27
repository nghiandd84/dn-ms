use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "withdrawals")]
#[dto(
    name(WithdrawalForCreate),
    columns(wallet_id, amount, payment_device_id, status, completed_at)
)]
#[dto(name(WithdrawalForUpdate), columns(status, completed_at), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: f32,
    pub payment_device_id: Uuid,
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub completed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::wallet::Entity",
        from = "Column::WalletId",
        to = "super::wallet::Column::Id"
    )]
    Wallet,
}

impl Related<super::wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wallet.def()
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
