use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "transactions")]
#[dto(
    name(TransactionForCreate),
    columns(wallet_id, transaction_type, amount, currency, status, description)
)]
#[dto(name(TransactionForUpdate), columns(status, description), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: String, // ENUM('DEPOSIT','WITHDRAWAL','TRANSFER','PAYMENT')
    pub amount: f32,              // Using String for Decimal compatibility
    pub currency: String,
    pub status: String, // ENUM('INITIATED', 'PENDING','SUCCESS','FAILED','CANCELLED')
    pub reference_id: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
