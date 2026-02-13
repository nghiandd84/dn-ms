use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "bookings")]
#[dto(
    name(BookingForCreate),
    columns(event_id, user_id, total_amount, status, booking_reference)
)]
#[dto(
    name(BookingForUpdate),
    columns(
        event_id,
        user_id,
        total_amount,
        status,
        payment_id,
        payment_status,
        confirmed_at
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: f32,
    pub status: String, // ENUM('PENDING','CONFIRMED','CANCELLED','FAILED') DEFAULT 'PENDING'
    pub payment_id: Uuid,
    pub payment_status: String, // ENUM('PENDING','SUCCESS','FAILED') DEFAULT 'PENDING'
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub booking_reference: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub confirmed_at: DateTime,
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
