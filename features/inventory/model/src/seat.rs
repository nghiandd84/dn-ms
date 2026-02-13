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

use features_inventory_entities::seat::{ModelOptionDto, SeatForCreateDto, SeatForUpdateDto};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct SeatData {
    pub id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub seat_number: Option<String>,
    pub section: Option<String>,
    pub row_number: Option<String>,
    pub seat_type: Option<String>,
    pub price: Option<f32>,
    pub status: Option<String>,
    pub version: Option<i32>,
    pub reserved_by: Option<String>,
    pub reserved_until: Option<DateTime>,
    pub booking_id: Option<Uuid>,
    pub created_at: Option<DateTime>,
}

impl Into<SeatData> for ModelOptionDto {
    fn into(self) -> SeatData {
        SeatData {
            id: self.id,
            event_id: self.event_id,
            seat_number: self.seat_number,
            section: self.section,
            row_number: self.row_number,
            seat_type: self.seat_type,
            price: self.price,
            status: self.status,
            version: self.version,
            reserved_by: self.reserved_by,
            reserved_until: self.reserved_until,
            booking_id: self.booking_id,
            created_at: self.created_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct SeatForCreateRequest {
    pub event_id: Uuid,
    pub seat_number: String,
    pub section: Option<String>,
    pub row_number: Option<String>,
    pub seat_type: Option<String>,
    pub price: f32,
}

impl Into<SeatForCreateDto> for SeatForCreateRequest {
    fn into(self) -> SeatForCreateDto {
        SeatForCreateDto {
            event_id: self.event_id,
            seat_number: self.seat_number,
            section: self.section.unwrap_or_default(),
            row_number: self.row_number.unwrap_or_default(),
            seat_type: self.seat_type.unwrap_or_else(|| "REGULAR".to_string()),
            price: self.price,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct SeatForUpdateRequest {
    pub seat_number: Option<String>,
    pub section: Option<String>,
    pub row_number: Option<String>,
    pub seat_type: Option<String>,
    pub price: Option<f32>,
    pub status: Option<String>,
    pub event_id: Option<Uuid>,
    pub version: Option<i32>,
    pub reserved_by: Option<String>,
    pub reserved_until: Option<DateTime>,
    pub booking_id: Option<Uuid>,
}

impl Into<SeatForUpdateDto> for SeatForUpdateRequest {
    fn into(self) -> SeatForUpdateDto {
        SeatForUpdateDto {
            seat_number: self.seat_number,
            section: self.section,
            row_number: self.row_number,
            seat_type: self.seat_type,
            price: self.price,
            status: self.status,
            event_id: self.event_id,
            version: self.version,
            reserved_by: self.reserved_by,
            reserved_until: self.reserved_until,
            booking_id: self.booking_id,
        }
    }
}
