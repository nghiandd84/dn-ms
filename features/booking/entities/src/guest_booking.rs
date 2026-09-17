use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::guest_booking_item;
use crate::guest_booking_item::Model as GuestBookingItemModel;

/// Guest booking record.
///
/// Holds a booking made by an unauthenticated guest. After the guest confirms
/// and pays (via OAuth), a guest booking is *promoted* into a real `bookings`
/// row (+ `booking_items`), and `promoted_booking_id` is set while `status`
/// transitions to PROMOTED.
///
/// What is being booked is expressed polymorphically via `resource_type` +
/// `resource_id` (mirroring the core `bookings` table), so a guest booking is
/// not limited to events — it can target rooms, cars, tables, appointments,
/// etc. `booking_type` classifies the booking and `booking_mode` selects the
/// reservation strategy applied on promotion (defaults to CAPACITY).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "guest_bookings")]
#[dto(
    name(GuestBookingForCreate),
    columns(
        booking_type,
        booking_mode,
        resource_type,
        resource_id,
        external_ref,
        site_origin,
        confirm_path,
        guest_email,
        guest_name,
        total_amount,
        currency,
        status,
        booking_reference,
        confirm_token_hash,
        expires_at,
        metadata
    )
)]
#[dto(
    name(GuestBookingForUpdate),
    columns(
        guest_email,
        guest_name,
        total_amount,
        currency,
        status,
        booking_reference,
        metadata,
        promoted_booking_id,
        confirmed_at,
        payment_expires_at
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    // classification
    /// What kind of booking this is, e.g. EVENT, HOTEL_ROOM, CAR_RENTAL,
    /// APPOINTMENT, WORKSHOP, ...
    pub booking_type: String,
    /// Reservation strategy applied when the guest booking is promoted into a
    /// real booking: WINDOW | CAPACITY | RECURRENCE | APPROVAL | QUEUE |
    /// DISPATCH. Defaults to CAPACITY.
    pub booking_mode: String,

    // polymorphic target (owned by another service)
    /// Kind of target, e.g. event, room, car, table, doctor, desk, ...
    #[sea_orm(nullable)]
    pub resource_type: Option<String>,
    /// Concrete target id in the owning service.
    #[sea_orm(nullable)]
    pub resource_id: Option<Uuid>,
    /// Opaque external identifier for the target when it has no UUID (non-native
    /// resource, e.g. a slug or vendor id). Used only when `resource_id` cannot
    /// carry the reference.
    #[sea_orm(nullable)]
    pub external_ref: Option<String>,

    /// Origin (scheme + host[:port]) of the front-end site the guest booked
    /// from, e.g. `https://site-a.com`. Validated against an allowlist at
    /// creation and used to build the confirmation link.
    pub site_origin: String,
    /// Path on the site where the guest confirms, e.g. `/path/confirm_booking`.
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub confirm_path: String,

    // guest identity (no auth account yet)
    pub guest_email: String,
    #[sea_orm(nullable)]
    pub guest_name: Option<String>,

    // money
    pub total_amount: f32,
    pub currency: String, // ISO 4217

    // lifecycle: PENDING, CONFIRMED, PROMOTED, CANCELLED, EXPIRED
    pub status: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub booking_reference: String,

    /// SHA-256 hash (hex) of the one-time confirmation token issued at creation.
    /// The plaintext token is returned once to the creator and required to
    /// confirm the booking. Only the hash is persisted.
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub confirm_token_hash: Option<String>,

    /// Deadline by which the guest must confirm. Set at creation to
    /// `created_at + CONFIRM_WINDOW`. After this instant an unconfirmed booking
    /// is rejected (and treated as EXPIRED).
    pub expires_at: DateTime,

    /// Deadline by which the guest must complete payment (promotion). Set at
    /// confirmation to `confirmed_at + PAYMENT_WINDOW`. After this instant an
    /// unpaid booking is rejected (and treated as PAYMENT_EXPIRED). Null until
    /// the booking is confirmed.
    #[sea_orm(nullable)]
    pub payment_expires_at: Option<DateTime>,

    // extensibility
    #[sea_orm(column_type = "Json", nullable)]
    pub metadata: Option<Json>,

    // set once promotion produces a real booking
    #[sea_orm(nullable)]
    pub promoted_booking_id: Option<Uuid>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(nullable)]
    pub confirmed_at: Option<DateTime>,

    /// Eager-loaded seat line items (populated via `?includes=items`).
    #[sea_orm(ignore)]
    pub items: Vec<GuestBookingItemModel>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "guest_booking_item::Entity")]
    GuestBookingItem,
}

impl Related<guest_booking_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuestBookingItem.def()
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
