use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_booking_entities::booking::{
    BookingForCreateDto, BookingForUpdateDto, ModelOptionDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BookingData {
    pub id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub payment_id: Option<Uuid>,
    pub payment_status: Option<String>,
    pub booking_reference: Option<String>,
    pub created_at: Option<DateTime>,
    pub confirmed_at: Option<DateTime>,
}

impl Into<BookingData> for ModelOptionDto {
    fn into(self) -> BookingData {
        BookingData {
            id: self.id,
            event_id: self.event_id,
            user_id: self.user_id,
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            payment_id: self.payment_id,
            payment_status: self.payment_status,
            booking_reference: self.booking_reference,
            created_at: self.created_at,
            confirmed_at: self.confirmed_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingForCreateRequest {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: f32,
    pub currency: String,
    pub status: String,
    pub booking_reference: String,
}

impl Into<BookingForCreateDto> for BookingForCreateRequest {
    fn into(self) -> BookingForCreateDto {
        BookingForCreateDto {
            event_id: self.event_id,
            user_id: self.user_id,
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            booking_reference: self.booking_reference,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingForUpdateRequest {
    pub event_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub total_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub payment_id: Option<Uuid>,
    pub payment_status: Option<String>,
    pub confirmed_at: Option<DateTime>,
}

impl Into<BookingForUpdateDto> for BookingForUpdateRequest {
    fn into(self) -> BookingForUpdateDto {
        BookingForUpdateDto {
            event_id: self.event_id,
            user_id: self.user_id,
            total_amount: self.total_amount,
            currency: self.currency,
            status: self.status,
            payment_id: self.payment_id,
            payment_status: self.payment_status,
            confirmed_at: self.confirmed_at,
        }
    }
}
