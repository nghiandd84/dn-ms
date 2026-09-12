use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::booking;

/// A concrete unit within a booking (generalizes the old `booking_seats`).
///
/// `item_type` + `item_id` point at the specific unit in its owning service:
/// a seat, table, room, car, desk, or time slot.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_items")]
#[dto(
    name(BookingItemForCreate),
    columns(booking_id, item_type, item_id, price, metadata)
)]
#[dto(
    name(BookingItemForUpdate),
    columns(booking_id, item_type, item_id, price, metadata),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub booking_id: Uuid,
    pub item_type: String, // seat, table, room, car, desk, slot
    pub item_id: Uuid,
    pub price: f32,
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "booking::Entity",
        from = "Column::BookingId",
        to = "booking::Column::Id"
    )]
    Booking,
}

impl Related<booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Booking.def()
    }
}

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
