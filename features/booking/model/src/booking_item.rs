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

use features_booking_entities::booking_item::{
    BookingItemForCreateDto, BookingItemForUpdateDto, Model, ModelOptionDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BookingItemData {
    pub id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub item_type: Option<String>,
    pub item_id: Option<Uuid>,
    pub price: Option<f32>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<Json>,
}

impl Into<BookingItemData> for ModelOptionDto {
    fn into(self) -> BookingItemData {
        BookingItemData {
            id: self.id,
            booking_id: self.booking_id,
            item_type: self.item_type,
            item_id: self.item_id,
            price: self.price,
            metadata: self.metadata.flatten(),
        }
    }
}

impl From<Model> for BookingItemData {
    fn from(m: Model) -> Self {
        BookingItemData {
            id: Some(m.id),
            booking_id: Some(m.booking_id),
            item_type: Some(m.item_type),
            item_id: Some(m.item_id),
            price: Some(m.price),
            metadata: m.metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingItemForCreateRequest {
    pub booking_id: Uuid,
    #[validate(length(
        min = 1,
        max = 50,
        code = "booking_item_type_length",
        message = "item_type must be between 1 and 50 characters"
    ))]
    pub item_type: String,
    pub item_id: Uuid,
    #[validate(range(
        min = 0.0,
        code = "booking_item_price_non_negative",
        message = "price must be >= 0"
    ))]
    pub price: f32,
    #[serde(default)]
    pub metadata: Option<Json>,
}

impl Into<BookingItemForCreateDto> for BookingItemForCreateRequest {
    fn into(self) -> BookingItemForCreateDto {
        BookingItemForCreateDto {
            booking_id: self.booking_id,
            item_type: self.item_type,
            item_id: self.item_id,
            price: self.price,
            metadata: self.metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingItemForUpdateRequest {
    pub booking_id: Option<Uuid>,
    pub item_type: Option<String>,
    pub item_id: Option<Uuid>,
    pub price: Option<f32>,
    #[serde(default)]
    pub metadata: Option<Json>,
}

impl Into<BookingItemForUpdateDto> for BookingItemForUpdateRequest {
    fn into(self) -> BookingItemForUpdateDto {
        BookingItemForUpdateDto {
            booking_id: self.booking_id,
            item_type: self.item_type,
            item_id: self.item_id,
            price: self.price,
            metadata: Some(self.metadata),
        }
    }
}
