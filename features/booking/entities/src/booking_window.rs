use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

use crate::booking;

/// WINDOW mode: time-window reservation of a specific unit.
/// (car rentals, equipment, hotel rooms, tables, desks, appointments)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_windows")]
#[dto(
    name(BookingWindowForCreate),
    columns(booking_id, starts_at, ends_at, party_size)
)]
#[dto(
    name(BookingWindowForUpdate),
    columns(starts_at, ends_at, party_size),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    pub starts_at: DateTime,
    pub ends_at: DateTime,
    #[sea_orm(nullable)]
    pub party_size: Option<i32>,
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
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}
