use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

use crate::booking;

/// RECURRENCE mode: repeating series or validity-period entitlements.
/// (weekly cleaning, recurring desk rentals, transit passes, permits)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_recurrence")]
#[dto(name(BookingRecurrenceForCreate), columns(booking_id, rrule, valid_from, valid_to))]
#[dto(name(BookingRecurrenceForUpdate), columns(rrule, valid_from, valid_to), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub rrule: Option<String>, // iCal RRULE; null for a plain validity period
    pub valid_from: DateTime,
    #[sea_orm(nullable)]
    pub valid_to: Option<DateTime>, // null = open-ended / renewable
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
