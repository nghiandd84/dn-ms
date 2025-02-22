use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::cake::{CakeForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"name": "NGHIA NGUYEN" , "bakery_id":1, "price": 1.0, "gluten_free": true, "serial": "123e4567-e89b-12d3-a456-426614174000"}))]
pub struct CakeForCreateRequest {
    #[validate(length(
        min = 1,
        max = 16,
        code = "name_length",
        message = "the length of email must be between 1 and 50"
    ))]
    pub name: String,
    pub bakery_id: i32,
    pub price: f64,
    pub gluten_free: bool,
    pub serial: Uuid,
}

impl Into<CakeForCreateDto> for CakeForCreateRequest {
    fn into(self) -> CakeForCreateDto {
        CakeForCreateDto {
            name: self.name,
            price: self.price,
            gluten_free: self.gluten_free,
            serial: self.serial,
            bakery_id: self.bakery_id,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct CakeData {
    id: Option<i32>,
    name: Option<String>,
    price: Option<f64>,
    gluten_free: Option<bool>,
    serial: Option<Uuid>,
}

impl Into<CakeData> for ModelOptionDto {
    fn into(self) -> CakeData {
        CakeData {
            id: self.id,
            name: self.name,
            price: self.price,
            gluten_free: self.gluten_free,
            serial: self.serial,
            ..Default::default()
        }
    }
}
