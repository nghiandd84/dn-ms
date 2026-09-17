use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use shared_shared_macro::Dto;

use crate::booking;

/// DISPATCH mode: on-demand fulfillment matched to a provider.
/// (food/courier delivery, emergency plumbing dispatch)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_dispatch")]
#[dto(
    name(BookingDispatchForCreate),
    columns(booking_id, dispatch_state, pickup_location, dropoff_location)
)]
#[dto(
    name(BookingDispatchForUpdate),
    columns(
        dispatch_state,
        provider_id,
        assigned_at,
        pickup_location,
        dropoff_location,
        eta
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    pub dispatch_state: String, // REQUESTED, ASSIGNED, EN_ROUTE, COMPLETED, FAILED
    #[sea_orm(nullable)]
    pub provider_id: Option<Uuid>, // assigned driver / technician
    #[sea_orm(nullable)]
    pub assigned_at: Option<DateTime>,
    #[sea_orm(column_type = "Json", nullable)]
    pub pickup_location: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub dropoff_location: Option<Json>,
    #[sea_orm(nullable)]
    pub eta: Option<DateTime>,
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
