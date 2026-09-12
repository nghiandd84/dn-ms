use chrono::NaiveDateTime as DateTime;
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

use features_booking_entities::booking::{BookingForCreateDto, BookingForUpdateDto, ModelOptionDto};

use super::booking_approval::{BookingApprovalData, BookingApprovalInput};
use super::booking_capacity::{BookingCapacityData, BookingCapacityInput};
use super::booking_dispatch::{BookingDispatchData, BookingDispatchInput};
use super::booking_item::BookingItemData;
use super::booking_queue::{BookingQueueData, BookingQueueInput};
use super::booking_recurrence::{BookingRecurrenceData, BookingRecurrenceInput};
use super::booking_window::{BookingWindowData, BookingWindowInput};

/// Booking behavior discriminator. Selects which mode-specific data and
/// reservation strategy apply. Serialized as an UPPER_SNAKE string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum BookingMode {
    Window,
    Capacity,
    Recurrence,
    Approval,
    Queue,
    Dispatch,
}

impl BookingMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BookingMode::Window => "WINDOW",
            BookingMode::Capacity => "CAPACITY",
            BookingMode::Recurrence => "RECURRENCE",
            BookingMode::Approval => "APPROVAL",
            BookingMode::Queue => "QUEUE",
            BookingMode::Dispatch => "DISPATCH",
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BookingData {
    pub id: Option<Uuid>,
    pub booking_type: Option<String>,
    pub booking_mode: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub payment_id: Option<Uuid>,
    pub payment_status: Option<String>,
    pub booking_reference: Option<String>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<Json>,
    pub version: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub confirmed_at: Option<DateTime>,

    // nested data (populated via ?includes=items)
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<Object>>)]
    pub items: Option<Vec<BookingItemData>>,

    // nested mode data (populated via ?includes=<mode>)
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub window: Option<BookingWindowData>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub capacity: Option<BookingCapacityData>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub recurrence: Option<BookingRecurrenceData>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub approval: Option<BookingApprovalData>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub queue: Option<BookingQueueData>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub dispatch: Option<BookingDispatchData>,
}

impl Into<BookingData> for ModelOptionDto {
    fn into(self) -> BookingData {
        BookingData {
            id: self.id,
            booking_type: self.booking_type,
            booking_mode: self.booking_mode,
            resource_type: self.resource_type.flatten(),
            resource_id: self.resource_id.flatten(),
            user_id: self.user_id,
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            payment_id: self.payment_id.flatten(),
            payment_status: self.payment_status,
            booking_reference: self.booking_reference,
            metadata: self.metadata.flatten(),
            version: self.version,
            created_at: self.created_at,
            updated_at: self.updated_at,
            confirmed_at: self.confirmed_at.flatten(),
            items: self
                .items
                .map(|items| items.into_iter().map(|i| i.into()).collect()),
            window: self.window.and_then(|v| v.into_iter().next().map(Into::into)),
            capacity: self
                .capacity
                .and_then(|v| v.into_iter().next().map(Into::into)),
            recurrence: self
                .recurrence
                .and_then(|v| v.into_iter().next().map(Into::into)),
            approval: self
                .approval
                .and_then(|v| v.into_iter().next().map(Into::into)),
            queue: self.queue.and_then(|v| v.into_iter().next().map(Into::into)),
            dispatch: self
                .dispatch
                .and_then(|v| v.into_iter().next().map(Into::into)),
        }
    }
}

/// Create request for a booking. Mode-specific data is supplied via the optional
/// `window` / `capacity` blocks; the service validates that the block matching
/// `booking_mode` is present.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingForCreateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "booking_type_length",
        message = "booking_type must be between 1 and 50 characters"
    ))]
    pub booking_type: String,
    pub booking_mode: BookingMode,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub user_id: Uuid,
    #[validate(range(
        min = 0.0,
        code = "booking_total_amount_non_negative",
        message = "total_amount must be >= 0"
    ))]
    pub total_amount: f32,
    #[validate(length(
        min = 3,
        max = 3,
        code = "booking_currency_length",
        message = "currency must be a 3-letter ISO code"
    ))]
    pub currency: String,
    #[validate(length(
        min = 1,
        max = 50,
        code = "booking_status_length",
        message = "status must be between 1 and 50 characters"
    ))]
    pub status: String,
    #[validate(length(
        min = 1,
        max = 100,
        code = "booking_reference_length",
        message = "booking_reference must be between 1 and 100 characters"
    ))]
    pub booking_reference: String,
    #[serde(default)]
    pub metadata: Option<Json>,

    // mode-specific blocks (only the one matching booking_mode is used)
    #[serde(default)]
    pub window: Option<BookingWindowInput>,
    #[serde(default)]
    pub capacity: Option<BookingCapacityInput>,
    #[serde(default)]
    pub recurrence: Option<BookingRecurrenceInput>,
    #[serde(default)]
    pub approval: Option<BookingApprovalInput>,
    #[serde(default)]
    pub queue: Option<BookingQueueInput>,
    #[serde(default)]
    pub dispatch: Option<BookingDispatchInput>,
}

impl BookingForCreateRequest {
    /// Build the core booking DTO (mode-specific blocks handled separately).
    pub fn to_core_dto(&self) -> BookingForCreateDto {
        BookingForCreateDto {
            booking_type: self.booking_type.clone(),
            booking_mode: self.booking_mode.as_str().to_string(),
            resource_type: self.resource_type.clone(),
            resource_id: self.resource_id,
            user_id: self.user_id,
            total_amount: self.total_amount,
            currency: self.currency.clone(),
            status: self.status.clone(),
            payment_status: "PENDING".to_string(),
            booking_reference: self.booking_reference.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingForUpdateRequest {
    pub booking_type: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub payment_id: Option<Uuid>,
    pub payment_status: Option<String>,
    #[serde(default)]
    pub metadata: Option<Json>,
    pub confirmed_at: Option<DateTime>,
}

impl Into<BookingForUpdateDto> for BookingForUpdateRequest {
    fn into(self) -> BookingForUpdateDto {
        BookingForUpdateDto {
            booking_type: self.booking_type,
            booking_mode: None,
            resource_type: Some(self.resource_type),
            resource_id: Some(self.resource_id),
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            payment_id: Some(self.payment_id),
            payment_status: self.payment_status,
            booking_reference: None,
            metadata: Some(self.metadata),
            confirmed_at: Some(self.confirmed_at),
        }
    }
}
