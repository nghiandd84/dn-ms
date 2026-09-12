use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_queue::{BookingQueueForCreateDto, Model, ModelOptionDto};

/// QUEUE-mode input block. The client supplies which queue to join; the queue
/// position is assigned by the service at insert time.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingQueueInput {
    pub queue_key: String,
}

impl BookingQueueInput {
    /// Build the DTO with a service-computed position.
    pub fn to_dto(&self, booking_id: Uuid, position: i32) -> BookingQueueForCreateDto {
        BookingQueueForCreateDto {
            booking_id,
            queue_key: self.queue_key.clone(),
            position: Some(position),
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingQueueData {
    pub booking_id: Option<Uuid>,
    pub queue_key: Option<String>,
    pub position: Option<i32>,
    pub estimated_ready: Option<DateTime>,
    pub offered_at: Option<DateTime>,
    pub hold_expires_at: Option<DateTime>,
}

impl Into<BookingQueueData> for ModelOptionDto {
    fn into(self) -> BookingQueueData {
        BookingQueueData {
            booking_id: self.booking_id,
            queue_key: self.queue_key,
            position: self.position.flatten(),
            estimated_ready: self.estimated_ready.flatten(),
            offered_at: self.offered_at.flatten(),
            hold_expires_at: self.hold_expires_at.flatten(),
        }
    }
}

impl From<Model> for BookingQueueData {
    fn from(m: Model) -> Self {
        BookingQueueData {
            booking_id: Some(m.booking_id),
            queue_key: Some(m.queue_key),
            position: m.position,
            estimated_ready: m.estimated_ready,
            offered_at: m.offered_at,
            hold_expires_at: m.hold_expires_at,
        }
    }
}
