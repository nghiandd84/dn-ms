use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::guest_booking;

/// A concrete seat unit within a guest booking (one row per selected seat).
///
/// Mirrors `booking_item`: `item_type` + `item_id` point at the specific unit
/// in its owning service (for events, `item_type` = "seat").
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "guest_booking_items")]
#[dto(
    name(GuestBookingItemForCreate),
    columns(guest_booking_id, item_type, item_id, price, metadata)
)]
#[dto(
    name(GuestBookingItemForUpdate),
    columns(guest_booking_id, item_type, item_id, price, metadata),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub guest_booking_id: Uuid,
    pub item_type: String, // seat, table, room, ...
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
        belongs_to = "guest_booking::Entity",
        from = "Column::GuestBookingId",
        to = "guest_booking::Column::Id"
    )]
    GuestBooking,
}

impl Related<guest_booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuestBooking.def()
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
