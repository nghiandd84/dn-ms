use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

use crate::booking;

/// APPROVAL mode: request-to-book / host-approval workflow.
/// (request-to-book rentals, emergency plumbing intake, gated beta access)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "booking_approval")]
#[dto(name(BookingApprovalForCreate), columns(booking_id, approval_status, hold_expires_at))]
#[dto(
    name(BookingApprovalForUpdate),
    columns(approval_status, approver_id, decided_at, hold_expires_at, reason),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
    pub approval_status: String, // PENDING, APPROVED, DECLINED, EXPIRED
    #[sea_orm(nullable)]
    pub approver_id: Option<Uuid>,
    pub requested_at: DateTime,
    #[sea_orm(nullable)]
    pub decided_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub hold_expires_at: Option<DateTime>,
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub reason: Option<String>,
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
        if insert {
            self.requested_at = ActiveValue::Set(Utc::now().naive_utc());
        }
        Ok(self)
    }
}
