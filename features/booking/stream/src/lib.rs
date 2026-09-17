use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Messages published to the booking Kafka topic.
///
/// `event_type` is the serde tag, so each variant serializes as
/// `{ "event_type": "<snake_case>", ... }`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum BookingMessage {
    /// Emitted when a guest booking is created. Carries the one-time confirmation
    /// token in plaintext so a downstream consumer can email it to the guest.
    /// The token is NOT persisted in plaintext and is NOT returned over HTTP.
    GuestBookingConfirmToken {
        message: GuestBookingConfirmTokenMessage,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuestBookingConfirmTokenMessage {
    pub guest_booking_id: Uuid,
    /// Kind of target being booked, e.g. event, room, car, ... (optional).
    pub resource_type: Option<String>,
    /// Concrete target id in the owning service (optional).
    pub resource_id: Option<Uuid>,
    /// Opaque external target reference when it has no UUID (optional).
    pub external_ref: Option<String>,
    pub guest_email: String,
    pub guest_name: Option<String>,
    pub booking_reference: String,
    /// One-time confirmation token (plaintext). Deliver out-of-band (email).
    pub confirm_token: String,
    /// Fully-built confirmation link the guest should click, e.g.
    /// `https://site-a.com/path/confirm_booking?guest_booking_id=...&token=...`.
    pub confirm_url: String,
    /// Deadline (RFC 3339 / ISO 8601) by which the guest must confirm.
    pub expires_at: String,
}

pub const PRODUCER_KEY: &str = "booking";
