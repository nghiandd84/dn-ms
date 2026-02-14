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

use features_event_entities::event::{EventForCreateDto, EventForUpdateDto, ModelOptionDto};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct EventData {
    pub id: Option<Uuid>,
    pub event_name: Option<String>,
    pub event_date: Option<DateTime>,
    pub venue_name: Option<String>,
    pub total_seats: Option<u32>,
    pub status: Option<String>,
    pub sale_start_time: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

impl Into<EventData> for ModelOptionDto {
    fn into(self) -> EventData {
        EventData {
            id: self.id,
            event_name: self.event_name,
            event_date: self.event_date,
            venue_name: self.venue_name,
            total_seats: self.total_seats,
            status: self.status,
            sale_start_time: self.sale_start_time,
            created_at: self.created_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct EventForCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "event_name",
        message = "the length of event_name must be between 1 and 255"
    ))]
    pub event_name: String,
    pub event_date: DateTime,
    pub venue_name: String,
    pub total_seats: u32,
    pub status: Option<String>,
    pub sale_start_time: Option<DateTime>,
}

impl Into<EventForCreateDto> for EventForCreateRequest {
    fn into(self) -> EventForCreateDto {
        EventForCreateDto {
            event_name: self.event_name,
            event_date: self.event_date,
            venue_name: self.venue_name,
            total_seats: self.total_seats,
            status: self.status.unwrap_or_else(|| "UPCOMING".to_string()),
            sale_start_time: self.sale_start_time.expect("Start time is required"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct EventForUpdateRequest {
    pub event_name: Option<String>,
    pub event_date: Option<DateTime>,
    pub venue_name: Option<String>,
    pub total_seats: Option<u32>,
    pub status: Option<String>,
    pub sale_start_time: Option<DateTime>,
}

impl Into<EventForUpdateDto> for EventForUpdateRequest {
    fn into(self) -> EventForUpdateDto {
        EventForUpdateDto {
            event_name: self.event_name,
            event_date: self.event_date,
            venue_name: self.venue_name,
            total_seats: self.total_seats,
            status: self.status,
            sale_start_time: self.sale_start_time,
        }
    }
}
