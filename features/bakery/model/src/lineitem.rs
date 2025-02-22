use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::lineitem::{LineitemForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"price":1.0,"quantity":1,"order_id":1,"cake_id":1}))]
pub struct LineitemForCreateRequest {
    pub price: f64,
    pub quantity: i32,
    pub order_id: i32,
    pub cake_id: i32,
}

impl Into<LineitemForCreateDto> for LineitemForCreateRequest {
    fn into(self) -> LineitemForCreateDto {
        LineitemForCreateDto {
            price: self.price,
            quantity: self.quantity,
            order_id: self.order_id,
            cake_id: self.cake_id,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct LineitemData {
    price: Option<f64>,
    quantity: Option<i32>,
    order_id: Option<i32>,
    cake_id: Option<i32>,
}

impl Into<LineitemData> for ModelOptionDto {
    fn into(self) -> LineitemData {
        LineitemData {
            price: self.price,
            quantity: self.quantity,
            order_id: self.order_id,
            cake_id: self.cake_id,
            ..Default::default()
        }
    }
}
