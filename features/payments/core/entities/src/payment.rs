use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "payments")]
#[dto(
    name(PaymentForCreate),
    columns(
        transaction_id,
        user_id,
        amount,
        currency,
        status,
        provider_name,
        gateway_transaction_id,
        idempotency_key
    )
)]
#[dto(
    name(PaymentForUpdate),
    columns(
        transaction_id,
        user_id,
        amount,
        currency,
        status,
        provider_name,
        gateway_transaction_id,
        idempotency_key
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub transaction_id: String,
    pub user_id: Uuid,
    pub amount: i64,
    #[sea_orm(column_type = "String(StringLen::N(3))")]
    pub currency: String,
    pub status: String, // ENUM('created','processing','succeeded','failed','refunded')
    #[sea_orm(column_type = "String(StringLen::N(50))", nullable)]
    pub provider_name: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable, unique)]
    pub gateway_transaction_id: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable, unique)]
    pub idempotency_key: String,
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
        self.updated_at = ActiveValue::Set(current_time);
        if insert {
            self.created_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}
