use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_booking_entities::guest_booking_item::{
    GuestBookingItemForCreateDto, GuestBookingItemForUpdateDto, Model, ModelOptionDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct GuestBookingItemData {
    pub id: Option<Uuid>,
    pub guest_booking_id: Option<Uuid>,
    pub item_type: Option<String>,
    pub item_id: Option<Uuid>,
    pub price: Option<f32>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<Json>,
}

impl Into<GuestBookingItemData> for ModelOptionDto {
    fn into(self) -> GuestBookingItemData {
        GuestBookingItemData {
            id: self.id,
            guest_booking_id: self.guest_booking_id,
            item_type: self.item_type,
            item_id: self.item_id,
            price: self.price,
            metadata: self.metadata.flatten(),
        }
    }
}

impl From<Model> for GuestBookingItemData {
    fn from(m: Model) -> Self {
        GuestBookingItemData {
            id: Some(m.id),
            guest_booking_id: Some(m.guest_booking_id),
            item_type: Some(m.item_type),
            item_id: Some(m.item_id),
            price: Some(m.price),
            metadata: m.metadata,
        }
    }
}

/// A single selected seat supplied inside a guest-booking create request.
/// One item per seat is created from these.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestSeatInput {
    /// Concrete unit id in the owning service (a seat id).
    pub item_id: Uuid,
    /// Unit type; defaults to "seat" when omitted.
    #[serde(default = "default_item_type")]
    #[validate(length(
        min = 1,
        max = 50,
        code = "guest_seat_item_type_length",
        message = "item_type must be between 1 and 50 characters"
    ))]
    pub item_type: String,
    #[validate(range(
        min = 0.0,
        code = "guest_seat_price_non_negative",
        message = "price must be >= 0"
    ))]
    pub price: f32,
    #[serde(default)]
    pub metadata: Option<Json>,
}

fn default_item_type() -> String {
    "seat".to_string()
}

impl GuestSeatInput {
    pub fn to_dto(&self, guest_booking_id: Uuid) -> GuestBookingItemForCreateDto {
        GuestBookingItemForCreateDto {
            guest_booking_id,
            item_type: self.item_type.clone(),
            item_id: self.item_id,
            price: self.price,
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestBookingItemForUpdateRequest {
    pub guest_booking_id: Option<Uuid>,
    pub item_type: Option<String>,
    pub item_id: Option<Uuid>,
    pub price: Option<f32>,
    #[serde(default)]
    pub metadata: Option<Json>,
}

impl Into<GuestBookingItemForUpdateDto> for GuestBookingItemForUpdateRequest {
    fn into(self) -> GuestBookingItemForUpdateDto {
        GuestBookingItemForUpdateDto {
            guest_booking_id: self.guest_booking_id,
            item_type: self.item_type,
            item_id: self.item_id,
            price: self.price,
            metadata: Some(self.metadata),
        }
    }
}
