use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "top_up_transactions")]
#[dto(
    name(TopUpTransactionForCreate),
    columns(
        wallet_id,
        amount,
        method,
        payment_provider_id,
        payment_transaction_id,
        status
    )
)]
#[dto(name(TopUpTransactionForUpdate), columns(status, completed_at), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: f32,
    pub method: String, // ENUM('CARD', 'UPI', 'BANK_TRANSFER', 'CASH')
    pub payment_provider_id: String,
    pub payment_transaction_id: String,
    pub status: String, // ENUM('PENDING', 'SUCCESS', 'FAILED')
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub completed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

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
