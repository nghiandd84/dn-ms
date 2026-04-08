use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "stripe_refunds")]
#[dto(
    name(StripeRefundForCreate),
    columns(
        payment_id,
        stripe_refund_id,
        stripe_payment_intent_id,
        amount,
        currency,
        status,
        reason,
        metadata
    )
)]
#[dto(name(StripeRefundForUpdate), columns(status, metadata), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payment_id: Uuid, // Reference to core payment
    #[sea_orm(column_type = "String(StringLen::N(255))", unique)]
    pub stripe_refund_id: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub stripe_payment_intent_id: String,
    pub amount: i64,
    #[sea_orm(column_type = "String(StringLen::N(3))")]
    pub currency: String,
    pub status: String, // Stripe refund status: pending, succeeded, failed, canceled
    #[sea_orm(column_type = "String(StringLen::N(50))", nullable)]
    pub reason: String, // duplicate, fraudulent, requested_by_customer
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: serde_json::Value,
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
