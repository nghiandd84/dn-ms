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
        max = 16,
        code = "name_length",
        message = "the length of email must be between 1 and 50"
    ))]
    pub name: String,
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
