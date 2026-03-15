use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "stripe_api_logs")]
#[dto(
    name(StripeApiLogForCreate),
    columns(
        endpoint,
        method,
        request_body,
        response_body,
        status_code,
        error_message,
        stripe_request_id
    )
)]
#[dto(
    name(StripeApiLogForUpdate),
    columns(response_body, status_code, error_message),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(500))")]
    pub endpoint: String, // e.g., /v1/payment_intents
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub method: String, // GET, POST, etc.
    #[sea_orm(column_type = "Text", nullable)]
    pub request_body: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub response_body: String,
    pub status_code: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub stripe_request_id: String, // Stripe's request ID from response headers
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