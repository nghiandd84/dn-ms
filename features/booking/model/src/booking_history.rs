use chrono::NaiveDateTime as DateTime;
use serde::Serialize;
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_booking_entities::booking_history::{BookingHistoryForCreateDto, ModelOptionDto};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BookingHistoryData {
    pub id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub event_type: Option<String>,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub actor_id: Option<Uuid>,
    pub note: Option<String>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
}

impl Into<BookingHistoryData> for ModelOptionDto {
    fn into(self) -> BookingHistoryData {
        BookingHistoryData {
            id: self.id,
            booking_id: self.booking_id,
            event_type: self.event_type,
            from_status: self.from_status.flatten(),
            to_status: self.to_status.flatten(),
            actor_id: self.actor_id.flatten(),
            note: self.note.flatten(),
            metadata: self.metadata.flatten(),
            created_at: self.created_at,
        }
    }
}

/// Well-known event types recorded in the booking history log.
pub struct BookingHistoryEvent;

impl BookingHistoryEvent {
    pub const CREATED: &'static str = "CREATED";
    pub const STATUS_CHANGED: &'static str = "STATUS_CHANGED";
    pub const PAYMENT_UPDATED: &'static str = "PAYMENT_UPDATED";
    pub const CANCELLED: &'static str = "CANCELLED";
}

/// Builder for a history entry, produced by the service and persisted via the
/// repo inside the same transaction as the change it records.
pub struct BookingHistoryEntry {
    pub booking_id: Uuid,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub actor_id: Option<Uuid>,
    pub note: Option<String>,
    pub metadata: Option<Json>,
}

impl BookingHistoryEntry {
    pub fn new(booking_id: Uuid, event_type: &str) -> Self {
        Self {
            booking_id,
            event_type: event_type.to_string(),
            from_status: None,
            to_status: None,
            actor_id: None,
            note: None,
            metadata: None,
        }
    }

    pub fn with_status_change(mut self, from: Option<String>, to: Option<String>) -> Self {
        self.from_status = from;
        self.to_status = to;
        self
    }

    pub fn with_actor(mut self, actor_id: Option<Uuid>) -> Self {
        self.actor_id = actor_id;
        self
    }

    pub fn with_note(mut self, note: Option<String>) -> Self {
        self.note = note;
        self
    }

    pub fn with_metadata(mut self, metadata: Option<Json>) -> Self {
        self.metadata = metadata;
        self
    }
}

impl Into<BookingHistoryForCreateDto> for BookingHistoryEntry {
    fn into(self) -> BookingHistoryForCreateDto {
        BookingHistoryForCreateDto {
            booking_id: self.booking_id,
            event_type: self.event_type,
            from_status: self.from_status,
            to_status: self.to_status,
            actor_id: self.actor_id,
            note: self.note,
            metadata: self.metadata,
        }
    }
}
