use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_booking_entities::booking_seat::{
    ModelOptionDto, BookingSeatForCreateDto, BookingSeatForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BookingSeatData {
    pub id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub seat_id: Option<Uuid>,
    pub price: Option<f32>,
}

impl Into<BookingSeatData> for ModelOptionDto {
    fn into(self) -> BookingSeatData {
        BookingSeatData {
            id: self.id,
            booking_id: self.booking_id,
            seat_id: self.seat_id,
            price: self.price,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingSeatForCreateRequest {
    pub booking_id: Uuid,
    pub seat_id: Uuid,
    pub price: f32,
}

impl Into<BookingSeatForCreateDto> for BookingSeatForCreateRequest {
    fn into(self) -> BookingSeatForCreateDto {
        BookingSeatForCreateDto {
            booking_id: self.booking_id,
            seat_id: self.seat_id,
            price: self.price,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct BookingSeatForUpdateRequest {
    pub booking_id: Option<Uuid>,
    pub seat_id: Option<Uuid>,
    pub price: Option<f32>,
}

impl Into<BookingSeatForUpdateDto> for BookingSeatForUpdateRequest {
    fn into(self) -> BookingSeatForUpdateDto {
        BookingSeatForUpdateDto {
            booking_id: self.booking_id,
            seat_id: self.seat_id,
            price: self.price,
        }
    }
}
