use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "payment_attempts")]
#[dto(
    name(PaymentAttemptForCreate),
    columns(
        payment_id,
        provider,
        raw_request,
        raw_response,
        success,
        error_message
    )
)]
#[dto(
    name(PaymentAttemptForUpdate),
    columns(
        payment_id,
        provider,
        raw_request,
        raw_response,
        success,
        error_message
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payment_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub provider: String,
    pub raw_request: Json,
    pub raw_response: Json,
    pub success: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: String,
    pub created_at: DateTime,
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
        // payment_attempts only have created_at
        if insert {
            self.created_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}
