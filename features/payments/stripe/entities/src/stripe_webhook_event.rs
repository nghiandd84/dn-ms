use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "stripe_webhook_events")]
#[dto(
    name(StripeWebhookEventForCreate),
    columns(
        stripe_event_id,
        event_type,
        event_data,
        processed,
        processing_error
    )
)]
#[dto(
    name(StripeWebhookEventForUpdate),
    columns(processed, processing_error),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))", unique)]
    pub stripe_event_id: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub event_type: String, // e.g., payment_intent.succeeded
    #[sea_orm(column_type = "Json")]
    pub event_data: serde_json::Value,
    pub processed: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub processing_error: String,
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