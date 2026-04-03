use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::bakery::{BakeryForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"name": "NGHIA NGUYEN","profit_margin":0.2}))]
pub struct BakeryForCreateRequest {
    #[validate(length(
        min = 1,
        max = 128,
        code = "name_length",
        message = "the length of name must be between 1 and 128"
    ))]
    pub name: String,
    #[validate(range(
        min = 0.0,
        max = 100.0,
        code = "bakery_profit_margin_range",
        message = "profit_margin must be between 0 and 100"
    ))]
    pub profit_margin: f64,
}

impl Into<BakeryForCreateDto> for BakeryForCreateRequest {
    fn into(self) -> BakeryForCreateDto {
        BakeryForCreateDto {
            name: self.name,
            profit_margin: self.profit_margin,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BakeryData {
    id: Option<i32>,
    name: Option<String>,
    profit_margin: Option<f64>,
}

impl Into<BakeryData> for ModelOptionDto {
    fn into(self) -> BakeryData {
        BakeryData {
            id: self.id,
            name: self.name,
            profit_margin: self.profit_margin,
            ..Default::default()
        }
    }
}
