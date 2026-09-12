use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_dispatch::{
    BookingDispatchForCreateDto, Model, ModelOptionDto,
};

/// DISPATCH-mode input block. A new dispatch always starts in `REQUESTED`;
/// the client supplies pickup/dropoff context as free-form JSON.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingDispatchInput {
    #[serde(default)]
    pub pickup_location: Option<Json>,
    #[serde(default)]
    pub dropoff_location: Option<Json>,
}

impl BookingDispatchInput {
    pub fn to_dto(&self, booking_id: Uuid) -> BookingDispatchForCreateDto {
        BookingDispatchForCreateDto {
            booking_id,
            dispatch_state: "REQUESTED".to_string(),
            pickup_location: self.pickup_location.clone(),
            dropoff_location: self.dropoff_location.clone(),
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingDispatchData {
    pub booking_id: Option<Uuid>,
    pub dispatch_state: Option<String>,
    pub provider_id: Option<Uuid>,
    pub assigned_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub pickup_location: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub dropoff_location: Option<Json>,
    pub eta: Option<DateTime>,
}

impl Into<BookingDispatchData> for ModelOptionDto {
    fn into(self) -> BookingDispatchData {
        BookingDispatchData {
            booking_id: self.booking_id,
            dispatch_state: self.dispatch_state,
            provider_id: self.provider_id.flatten(),
            assigned_at: self.assigned_at.flatten(),
            pickup_location: self.pickup_location.flatten(),
            dropoff_location: self.dropoff_location.flatten(),
            eta: self.eta.flatten(),
        }
    }
}

impl From<Model> for BookingDispatchData {
    fn from(m: Model) -> Self {
        BookingDispatchData {
            booking_id: Some(m.booking_id),
            dispatch_state: Some(m.dispatch_state),
            provider_id: m.provider_id,
            assigned_at: m.assigned_at,
            pickup_location: m.pickup_location,
            dropoff_location: m.dropoff_location,
            eta: m.eta,
        }
    }
}
