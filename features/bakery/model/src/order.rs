use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_data_core::deserialize::*;
use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::order::{ModelOptionDto, OrderForCreateDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"total":1.0,"bakery_id":1,"customer_id":1,"placed_at":"2023-10-01T00:00:00"}))]
pub struct OrderForCreateRequest {
    pub total: f64,
    pub bakery_id: i32,
    pub customer_id: i32,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub placed_at: DateTime,
}

impl Into<OrderForCreateDto> for OrderForCreateRequest {
    fn into(self) -> OrderForCreateDto {
        OrderForCreateDto {
            total: self.total,
            bakery_id: self.bakery_id,
            customer_id: self.customer_id,
            placed_at: self.placed_at,
            ..Default::default()
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct OrderData {
    id: Option<i32>,
    total: Option<f64>,
    bakery_id: Option<i32>,
    customer_id: Option<i32>,
    placed_at: Option<DateTime>,
}

impl Into<OrderData> for ModelOptionDto {
    fn into(self) -> OrderData {
        OrderData {
            id: self.id,
            total: self.total,
            bakery_id: self.bakery_id,
            customer_id: self.customer_id,
            placed_at: self.placed_at,
            ..Default::default()
        }
    }
}
