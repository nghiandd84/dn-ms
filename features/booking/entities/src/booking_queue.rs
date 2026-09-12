use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

use crate::booking;

/// QUEUE mode: waitlist / ordered position, no fixed time.
/// (restaurant waitlists, beta waitlist, request-to-book queues)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_queue")]
#[dto(name(BookingQueueForCreate), columns(booking_id, queue_key, position))]
#[dto(
    name(BookingQueueForUpdate),
    columns(position, estimated_ready, offered_at, hold_expires_at),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub queue_key: String, // which queue (restaurant id, product id)
    #[sea_orm(nullable)]
    pub position: Option<i32>,
    #[sea_orm(nullable)]
    pub estimated_ready: Option<DateTime>,
    #[sea_orm(nullable)]
    pub offered_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub hold_expires_at: Option<DateTime>, // offer expiry if not accepted
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
