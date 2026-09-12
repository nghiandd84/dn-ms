use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_recurrence::{
    BookingRecurrenceForCreateDto, Model, ModelOptionDto,
};

/// RECURRENCE-mode input block supplied inside a create-booking request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingRecurrenceInput {
    /// iCal RRULE string; omit for a plain validity-period entitlement.
    pub rrule: Option<String>,
    pub valid_from: DateTime,
    pub valid_to: Option<DateTime>,
}

impl BookingRecurrenceInput {
    pub fn to_dto(&self, booking_id: Uuid) -> BookingRecurrenceForCreateDto {
        BookingRecurrenceForCreateDto {
            booking_id,
            rrule: self.rrule.clone(),
            valid_from: self.valid_from,
            valid_to: self.valid_to,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingRecurrenceData {
    pub booking_id: Option<Uuid>,
    pub rrule: Option<String>,
    pub valid_from: Option<DateTime>,
    pub valid_to: Option<DateTime>,
}

impl Into<BookingRecurrenceData> for ModelOptionDto {
    fn into(self) -> BookingRecurrenceData {
        BookingRecurrenceData {
            booking_id: self.booking_id,
            rrule: self.rrule.flatten(),
            valid_from: self.valid_from,
            valid_to: self.valid_to.flatten(),
        }
    }
}

impl From<Model> for BookingRecurrenceData {
    fn from(m: Model) -> Self {
        BookingRecurrenceData {
            booking_id: Some(m.booking_id),
            rrule: m.rrule,
            valid_from: Some(m.valid_from),
            valid_to: m.valid_to,
        }
    }
}
