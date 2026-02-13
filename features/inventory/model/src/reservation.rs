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

use features_inventory_entities::reservation::{
    ModelOptionDto, ReservationForCreateDto, ReservationForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct ReservationData {
    pub id: Option<Uuid>,
    pub seat_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub expires_at: Option<DateTime>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
}
impl Into<ReservationData> for ModelOptionDto {
    fn into(self) -> ReservationData {
        ReservationData {
            id: self.id,
            seat_id: self.seat_id,
            event_id: self.event_id,
            user_id: self.user_id,
            expires_at: self.expires_at,
            status: self.status,
            created_at: self.created_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReservationForCreateRequest {
    pub seat_id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime,
    pub status: String,
}

impl Into<ReservationForCreateDto> for ReservationForCreateRequest {
    fn into(self) -> ReservationForCreateDto {
        ReservationForCreateDto {
            seat_id: self.seat_id,
            event_id: self.event_id,
            user_id: self.user_id,
            expires_at: self.expires_at,
            status: self.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReservationForUpdateRequest {
    pub seat_id: Option<Uuid>,
    pub event_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub expires_at: Option<DateTime>,
    pub status: Option<String>,
}

impl Into<ReservationForUpdateDto> for ReservationForUpdateRequest {
    fn into(self) -> ReservationForUpdateDto {
        ReservationForUpdateDto {
            seat_id: self.seat_id,
            event_id: self.event_id,
            user_id: self.user_id,
            expires_at: self.expires_at,
            status: self.status,
        }
    }
}
