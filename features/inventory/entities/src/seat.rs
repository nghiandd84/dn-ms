use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "seats")]
#[dto(
    name(SeatForCreate),
    columns(event_id, seat_number, section, row_number, seat_type, price)
)]
#[dto(
    name(SeatForUpdate),
    columns(
        event_id,
        seat_number,
        section,
        row_number,
        seat_type,
        price,
        version,
        status,
        reserved_by,
        reserved_until,
        booking_id
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub event_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub seat_number: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub section: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub row_number: String,
    pub seat_type: String, // ENUM('REGULAR','VIP','PREMIUM') DEFAULT 'REGULAR'
    pub price: i32,
    pub version: i32,
    pub status: String, // ENUM('AVAILABLE','RESERVED','BOOKED','BLOCKED') DEFAULT 'AVAILABLE'
    pub reserved_by: String, // -- User ID or session ID
    pub reserved_until: DateTime,
    pub booking_id: Uuid,
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
