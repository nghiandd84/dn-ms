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
    pub event_id: Option<Uuid>,
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
            event_id: self.event_id,
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
    pub event_id: Uuid,
    /// Origin (scheme + host[:port]) of the site the guest is booking from,
    /// e.g. `https://site-a.com`. Must be on the server's allowlist.
    #[validate(url(code = "site_origin_invalid", message = "site_origin must be a valid URL"))]
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
    #[validate(email(code = "guest_email_invalid", message = "guest_email must be a valid email"))]
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
            event_id: self.event_id,
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
