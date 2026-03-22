use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "webhooks")]
#[dto(
    name(WebhookForCreate),
    columns(merchant_id, url, event_types, secret, status)
)]
#[dto(
    name(WebhookForUpdate),
    columns(url, event_types, secret, status),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub merchant_id: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub url: String,

    #[sea_orm(column_type = "String(StringLen::N(128))", array)]
    pub event_types: Vec<String>, // e.g. PAYMENT_SUCCEEDED, PAYMENT_FAILED

    // Used for validating webhook signatures
    // In a real application, this should be stored securely and not in plaintext
    // On each event POSTed to the webhook URL, the system would
    // - compute HMAC of the payload with this secret and compare with signature header
    // - set headers like `X-Webhook-Signature` to allow the receiver to verify authenticity
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub secret: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::merchant::Entity",
        from = "Column::MerchantId",
        to = "super::merchant::Column::Id"
    )]
    Merchant,
}

impl Related<super::merchant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Merchant.def()
    }
}

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
