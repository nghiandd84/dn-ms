use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

use crate::booking;

/// CAPACITY mode: finite slots/seats in a scheduled container.
/// (airline seats, workshops, event tickets, reserved transit seats)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_capacity")]
#[dto(
    name(BookingCapacityForCreate),
    columns(booking_id, container_id, quantity)
)]
#[dto(
    name(BookingCapacityForUpdate),
    columns(container_id, quantity),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    pub container_id: Uuid, // flight, workshop session, event, trip
    pub quantity: i32,
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
