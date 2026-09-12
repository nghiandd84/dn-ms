use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

/// Append-only lifecycle event log for a booking.
///
/// One row per meaningful change (creation, status change, payment update,
/// cancellation, ...). Written transactionally with the mutation it records so
/// the timeline never drifts from the booking state.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_history")]
#[dto(
    name(BookingHistoryForCreate),
    columns(
        booking_id,
        event_type,
        from_status,
        to_status,
        actor_id,
        note,
        metadata
    )
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub booking_id: Uuid,
    pub event_type: String, // CREATED, STATUS_CHANGED, PAYMENT_UPDATED, CANCELLED, ...
    #[sea_orm(nullable)]
    pub from_status: Option<String>,
    #[sea_orm(nullable)]
    pub to_status: Option<String>,
    #[sea_orm(nullable)]
    pub actor_id: Option<Uuid>,
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub note: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,
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
        if insert {
            self.created_at = ActiveValue::Set(Utc::now().naive_utc());
        }
        Ok(self)
    }
}
