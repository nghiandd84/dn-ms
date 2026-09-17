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

use features_booking_entities::guest_booking::{
    GuestBookingForCreateDto, GuestBookingForUpdateDto, ModelOptionDto,
};

use super::booking::BookingMode;
use super::guest_booking_item::{GuestBookingItemData, GuestSeatInput};

/// Guest-booking lifecycle status values.
pub struct GuestBookingStatus;

impl GuestBookingStatus {
    pub const PENDING: &'static str = "PENDING";
    pub const CONFIRMED: &'static str = "CONFIRMED";
    pub const PROMOTED: &'static str = "PROMOTED";
    pub const CANCELLED: &'static str = "CANCELLED";
    pub const EXPIRED: &'static str = "EXPIRED";
    pub const PAYMENT_EXPIRED: &'static str = "PAYMENT_EXPIRED";
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct GuestBookingData {
    pub id: Option<Uuid>,
    pub booking_type: Option<String>,
    pub booking_mode: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub external_ref: Option<String>,
    pub site_origin: Option<String>,
    pub confirm_path: Option<String>,
    pub guest_email: Option<String>,
    pub guest_name: Option<String>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub booking_reference: Option<String>,
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<Json>,
    pub promoted_booking_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub confirmed_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    pub payment_expires_at: Option<DateTime>,

    // nested data (populated via ?includes=items)
    #[skip_param]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<Object>>)]
    pub items: Option<Vec<GuestBookingItemData>>,
}

impl Into<GuestBookingData> for ModelOptionDto {
    fn into(self) -> GuestBookingData {
        GuestBookingData {
            id: self.id,
            booking_type: self.booking_type,
            booking_mode: self.booking_mode,
            resource_type: self.resource_type.flatten(),
            resource_id: self.resource_id.flatten(),
            external_ref: self.external_ref.flatten(),
            site_origin: self.site_origin,
            confirm_path: self.confirm_path,
            guest_email: self.guest_email,
            guest_name: self.guest_name.flatten(),
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            booking_reference: self.booking_reference,
            metadata: self.metadata.flatten(),
            promoted_booking_id: self.promoted_booking_id.flatten(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            confirmed_at: self.confirmed_at.flatten(),
            expires_at: self.expires_at,
            payment_expires_at: self.payment_expires_at.flatten(),
            items: self
                .items
                .map(|items| items.into_iter().map(|i| i.into()).collect()),
        }
    }
}

/// Create request for a guest booking. One booking item is created per entry in
/// `seats`. `total_amount` is computed by the service from the seat prices.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestBookingForCreateRequest {
    /// What kind of booking this is, e.g. EVENT, HOTEL_ROOM, CAR_RENTAL,
    /// APPOINTMENT, ... Defaults to EVENT for backwards compatibility.
    #[serde(default = "default_booking_type")]
    #[validate(length(
        min = 1,
        max = 50,
        code = "guest_booking_type_length",
        message = "booking_type must be between 1 and 50 characters"
    ))]
    pub booking_type: String,
    /// Reservation strategy applied on promotion. Defaults to CAPACITY.
    #[serde(default)]
    pub booking_mode: BookingMode,
    /// Kind of target, e.g. event, room, car, table, ... Optional.
    pub resource_type: Option<String>,
    /// Concrete target id in the owning service. Optional.
    pub resource_id: Option<Uuid>,
    /// Opaque external identifier for the target when it has no UUID; use
    /// instead of `resource_id` for non-native resources. Optional.
    pub external_ref: Option<String>,
    /// Origin (scheme + host[:port]) of the site the guest is booking from,
    /// e.g. `https://site-a.com`. Must be on the server's allowlist.
    #[validate(url(
        code = "site_origin_invalid",
        message = "site_origin must be a valid URL"
    ))]
    pub site_origin: String,
    /// Path where the guest confirms, e.g. `/path/confirm_booking`.
    /// Must start with `/`. Defaults to `/path/confirm_booking`.
    #[serde(default = "default_confirm_path")]
    #[validate(length(
        min = 1,
        max = 255,
        code = "confirm_path_length",
        message = "confirm_path must be between 1 and 255 characters"
    ))]
    pub confirm_path: String,
    #[validate(email(
        code = "guest_email_invalid",
        message = "guest_email must be a valid email"
    ))]
    pub guest_email: String,
    pub guest_name: Option<String>,
    #[validate(length(
        min = 3,
        max = 3,
        code = "guest_booking_currency_length",
        message = "currency must be a 3-letter ISO code"
    ))]
    pub currency: String,
    #[serde(default)]
    pub metadata: Option<Json>,
    /// One seat per entry; at least one is required.
    #[validate(length(
        min = 1,
        code = "guest_booking_seats_required",
        message = "at least one seat is required"
    ))]
    #[validate(nested)]
    pub seats: Vec<GuestSeatInput>,
}

fn default_confirm_path() -> String {
    "/path/confirm_booking".to_string()
}

fn default_booking_type() -> String {
    "EVENT".to_string()
}

impl GuestBookingForCreateRequest {
    /// Build the core guest-booking DTO. `total_amount`, `booking_reference`,
    /// `confirm_token_hash` and `expires_at` are provided by the service
    /// (computed / generated).
    pub fn to_core_dto(
        &self,
        total_amount: f32,
        booking_reference: String,
        confirm_token_hash: String,
        expires_at: DateTime,
    ) -> GuestBookingForCreateDto {
        GuestBookingForCreateDto {
            booking_type: self.booking_type.clone(),
            booking_mode: self.booking_mode.as_str().to_string(),
            resource_type: self.resource_type.clone(),
            resource_id: self.resource_id,
            external_ref: self.external_ref.clone(),
            site_origin: self.site_origin.clone(),
            confirm_path: self.confirm_path.clone(),
            guest_email: self.guest_email.clone(),
            guest_name: self.guest_name.clone(),
            total_amount,
            currency: self.currency.clone(),
            status: GuestBookingStatus::PENDING.to_string(),
            booking_reference,
            confirm_token_hash: Some(confirm_token_hash),
            expires_at,
            metadata: self.metadata.clone(),
        }
    }
}

/// Admin create request for a guest booking. Unlike the public create flow,
/// this bypasses the confirmation-token / Kafka email step: an administrator
/// creates the record directly with an explicit `status`. No confirm token is
/// generated or stored.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestBookingAdminForCreateRequest {
    /// What kind of booking this is, e.g. EVENT, HOTEL_ROOM, CAR_RENTAL,
    /// APPOINTMENT, ... Defaults to EVENT.
    #[serde(default = "default_booking_type")]
    #[validate(length(
        min = 1,
        max = 50,
        code = "guest_booking_type_length",
        message = "booking_type must be between 1 and 50 characters"
    ))]
    pub booking_type: String,
    /// Reservation strategy applied on promotion. Defaults to CAPACITY.
    #[serde(default)]
    pub booking_mode: BookingMode,
    /// Kind of target, e.g. event, room, car, table, ... Optional.
    pub resource_type: Option<String>,
    /// Concrete target id in the owning service. Optional.
    pub resource_id: Option<Uuid>,
    /// Opaque external identifier for the target when it has no UUID; use
    /// instead of `resource_id` for non-native resources. Optional.
    pub external_ref: Option<String>,
    #[validate(url(
        code = "site_origin_invalid",
        message = "site_origin must be a valid URL"
    ))]
    pub site_origin: String,
    #[serde(default = "default_confirm_path")]
    #[validate(length(
        min = 1,
        max = 255,
        code = "confirm_path_length",
        message = "confirm_path must be between 1 and 255 characters"
    ))]
    pub confirm_path: String,
    #[validate(email(
        code = "guest_email_invalid",
        message = "guest_email must be a valid email"
    ))]
    pub guest_email: String,
    pub guest_name: Option<String>,
    #[validate(range(
        min = 0.0,
        code = "guest_booking_total_amount_non_negative",
        message = "total_amount must be >= 0"
    ))]
    pub total_amount: f32,
    #[validate(length(
        min = 3,
        max = 3,
        code = "guest_booking_currency_length",
        message = "currency must be a 3-letter ISO code"
    ))]
    pub currency: String,
    /// Lifecycle status, e.g. PENDING, CONFIRMED, CANCELLED. Defaults to PENDING.
    #[serde(default = "default_admin_status")]
    #[validate(length(
        min = 1,
        max = 50,
        code = "guest_booking_status_length",
        message = "status must be between 1 and 50 characters"
    ))]
    pub status: String,
    /// Optional human-readable reference. If omitted, the service generates one.
    pub booking_reference: Option<String>,
    /// Confirmation deadline. If omitted, the service applies a default window.
    pub expires_at: Option<DateTime>,
    #[serde(default)]
    pub metadata: Option<Json>,
}

fn default_admin_status() -> String {
    GuestBookingStatus::PENDING.to_string()
}

impl GuestBookingAdminForCreateRequest {
    /// Build the core guest-booking DTO for admin creation. `booking_reference`
    /// and `expires_at` are provided by the service when not supplied here.
    pub fn to_core_dto(
        &self,
        booking_reference: String,
        expires_at: DateTime,
    ) -> GuestBookingForCreateDto {
        GuestBookingForCreateDto {
            booking_type: self.booking_type.clone(),
            booking_mode: self.booking_mode.as_str().to_string(),
            resource_type: self.resource_type.clone(),
            resource_id: self.resource_id,
            external_ref: self.external_ref.clone(),
            site_origin: self.site_origin.clone(),
            confirm_path: self.confirm_path.clone(),
            guest_email: self.guest_email.clone(),
            guest_name: self.guest_name.clone(),
            total_amount: self.total_amount,
            currency: self.currency.clone(),
            status: self.status.clone(),
            booking_reference,
            // Admin-created bookings carry no one-time confirmation token.
            confirm_token_hash: None,
            expires_at,
            metadata: self.metadata.clone(),
        }
    }
}

/// Confirm request for a guest booking. Requires the one-time `confirm_token`
/// issued at creation; this authorizes the (unauthenticated) confirmation. The
/// token is delivered out-of-band (emailed to the guest via a Kafka consumer).
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestBookingConfirmRequest {
    #[validate(length(
        min = 1,
        code = "guest_confirm_token_required",
        message = "confirm_token is required"
    ))]
    pub confirm_token: String,
    #[serde(default)]
    pub metadata: Option<Json>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GuestBookingForUpdateRequest {
    pub guest_email: Option<String>,
    pub guest_name: Option<String>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub booking_reference: Option<String>,
    #[serde(default)]
    pub metadata: Option<Json>,
    pub promoted_booking_id: Option<Uuid>,
    pub confirmed_at: Option<DateTime>,
    pub payment_expires_at: Option<DateTime>,
}

impl Into<GuestBookingForUpdateDto> for GuestBookingForUpdateRequest {
    fn into(self) -> GuestBookingForUpdateDto {
        GuestBookingForUpdateDto {
            guest_email: self.guest_email,
            guest_name: Some(self.guest_name),
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            booking_reference: self.booking_reference,
            metadata: Some(self.metadata),
            promoted_booking_id: Some(self.promoted_booking_id),
            confirmed_at: Some(self.confirmed_at),
            payment_expires_at: Some(self.payment_expires_at),
        }
    }
}
