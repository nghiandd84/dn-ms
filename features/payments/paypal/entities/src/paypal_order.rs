use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "paypal_orders")]
#[dto(
    name(PaypalOrderForCreate),
    columns(
        payment_id,
        paypal_order_id,
        amount,
        currency,
        status,
        approval_url,
        capture_id,
        metadata
    )
)]
#[dto(name(PaypalOrderForUpdate), columns(status, capture_id, metadata), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payment_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub paypal_order_id: String,
    pub amount: i64,
    #[sea_orm(column_type = "String(StringLen::N(3))")]
    pub currency: String,
    pub status: String,
    #[sea_orm(column_type = "String(StringLen::N(2000))", nullable)]
    pub approval_url: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub capture_id: String,
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
