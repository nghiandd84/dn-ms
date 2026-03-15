use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "stripe_payment_intents")]
#[dto(
    name(StripePaymentIntentForCreate),
    columns(
        payment_id,
        stripe_payment_intent_id,
        amount,
        currency,
        status,
        client_secret,
        metadata
    )
)]
#[dto(name(StripePaymentIntentForUpdate), columns(status, metadata), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    // Reference to core payment
    pub payment_id: Uuid,
    // Create with null, then update after Stripe creates the PaymentIntent and we have the ID
    // example: "pi_1J2Y3Z4A5B6C7D8E9F0G1H2I3"
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub stripe_payment_intent_id: String,
    pub amount: i64,
    #[sea_orm(column_type = "String(StringLen::N(3))")]
    pub currency: String,
    // Stripe status: requires_payment_method, requires_confirmation, processing, requires_action, canceled, succeeded
    pub status: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub client_secret: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Json,
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
