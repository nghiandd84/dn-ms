use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_window::{
    BookingWindowForCreateDto, Model, ModelOptionDto,
};

/// WINDOW-mode input block supplied inside a create-booking request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingWindowInput {
    pub starts_at: DateTime,
    pub ends_at: DateTime,
    pub party_size: Option<i32>,
}

impl BookingWindowInput {
    pub fn to_dto(&self, booking_id: Uuid) -> BookingWindowForCreateDto {
        BookingWindowForCreateDto {
            booking_id,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            party_size: self.party_size,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingWindowData {
    pub booking_id: Option<Uuid>,
    pub starts_at: Option<DateTime>,
    pub ends_at: Option<DateTime>,
    pub party_size: Option<i32>,
}

impl Into<BookingWindowData> for ModelOptionDto {
    fn into(self) -> BookingWindowData {
        BookingWindowData {
            booking_id: self.booking_id,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            party_size: self.party_size.flatten(),
        }
    }
}

impl From<Model> for BookingWindowData {
    fn from(m: Model) -> Self {
        BookingWindowData {
            booking_id: Some(m.booking_id),
            starts_at: Some(m.starts_at),
            ends_at: Some(m.ends_at),
            party_size: m.party_size,
        }
    }
}
