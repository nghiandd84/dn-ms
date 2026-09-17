use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::booking_approval::Model as BookingApprovalModel;
use crate::booking_capacity::Model as BookingCapacityModel;
use crate::booking_dispatch::Model as BookingDispatchModel;
use crate::booking_item;
use crate::booking_item::Model as BookingItemModel;
use crate::booking_queue::Model as BookingQueueModel;
use crate::booking_recurrence::Model as BookingRecurrenceModel;
use crate::booking_window::Model as BookingWindowModel;
use crate::{
    booking_approval, booking_capacity, booking_dispatch, booking_queue, booking_recurrence,
    booking_window,
};

/// Core booking record.
///
/// Holds the shared lifecycle, ownership and money fields common to ALL booking
/// types. What is being booked is expressed polymorphically via
/// `resource_type` + `resource_id`, and *how* the booking behaves is selected by
/// `booking_mode`, which points at exactly one mode child table
/// (`booking_window`, `booking_capacity`, `booking_recurrence`,
/// `booking_approval`, `booking_queue`, `booking_dispatch`).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "bookings")]
#[dto(
    name(BookingForCreate),
    columns(
        booking_type,
        booking_mode,
        resource_type,
        resource_id,
        external_ref,
        user_id,
        total_amount,
        currency,
        status,
        payment_status,
        booking_reference,
        metadata
    )
)]
#[dto(
    name(BookingForUpdate),
    columns(
        booking_type,
        booking_mode,
        resource_type,
        resource_id,
        external_ref,
        total_amount,
        currency,
        status,
        payment_id,
        payment_status,
        booking_reference,
        metadata,
        confirmed_at
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    // classification
    pub booking_type: String, // EVENT, HOTEL_ROOM, CAR_RENTAL, APPOINTMENT, WORKSHOP, WAITLIST, ...
    pub booking_mode: String, // WINDOW | CAPACITY | RECURRENCE | APPROVAL | QUEUE | DISPATCH

    // polymorphic target (owned by another service)
    #[sea_orm(nullable)]
    pub resource_type: Option<String>, // event, room, car, table, doctor, desk, garage, ...
    #[sea_orm(nullable)]
    pub resource_id: Option<Uuid>,
    /// Opaque external identifier for the target when it has no UUID (non-native
    /// resource, e.g. a slug or vendor id). Used only when `resource_id` cannot
    /// carry the reference; exactly one of `resource_id` / `external_ref` is
    /// expected to identify the target.
    #[sea_orm(nullable)]
    pub external_ref: Option<String>,

    // ownership
    pub user_id: Uuid,

    // money (minor units are recommended, kept as f32 to match legacy price columns)
    pub total_amount: f32,
    pub currency: String, // ISO 4217

    // lifecycle
    pub status: String, // PENDING, CONFIRMED, CANCELLED, FAILED
    #[sea_orm(nullable)]
    pub payment_id: Option<Uuid>,
    pub payment_status: String, // PENDING, SUCCESS, FAILED
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub booking_reference: String,

    // extensibility + concurrency
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,
    pub version: i32, // optimistic locking

    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(nullable)]
    pub confirmed_at: Option<DateTime>,

    /// Eager-loaded line items (populated via `?includes=items`).
    #[sea_orm(ignore)]
    pub items: Vec<BookingItemModel>,

    // Eager-loaded mode child rows (populated via `?includes=<mode>`).
    // Each is 0-or-1 row; SeaORM's find_with_related returns a Vec, so these
    // are Vec and the model layer takes the first element.
    #[sea_orm(ignore)]
    pub window: Vec<BookingWindowModel>,
    #[sea_orm(ignore)]
    pub capacity: Vec<BookingCapacityModel>,
    #[sea_orm(ignore)]
    pub recurrence: Vec<BookingRecurrenceModel>,
    #[sea_orm(ignore)]
    pub approval: Vec<BookingApprovalModel>,
    #[sea_orm(ignore)]
    pub queue: Vec<BookingQueueModel>,
    #[sea_orm(ignore)]
    pub dispatch: Vec<BookingDispatchModel>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "booking_item::Entity")]
    BookingItem,
    #[sea_orm(has_many = "booking_window::Entity")]
    BookingWindow,
    #[sea_orm(has_many = "booking_capacity::Entity")]
    BookingCapacity,
    #[sea_orm(has_many = "booking_recurrence::Entity")]
    BookingRecurrence,
    #[sea_orm(has_many = "booking_approval::Entity")]
    BookingApproval,
    #[sea_orm(has_many = "booking_queue::Entity")]
    BookingQueue,
    #[sea_orm(has_many = "booking_dispatch::Entity")]
    BookingDispatch,
}

impl Related<booking_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingItem.def()
    }
}

impl Related<booking_window::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingWindow.def()
    }
}

impl Related<booking_capacity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingCapacity.def()
    }
}

impl Related<booking_recurrence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingRecurrence.def()
    }
}

impl Related<booking_approval::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingApproval.def()
    }
}

impl Related<booking_queue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingQueue.def()
    }
}

impl Related<booking_dispatch::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingDispatch.def()
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
