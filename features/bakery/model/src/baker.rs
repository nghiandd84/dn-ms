use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use super::bakery::{BakeryData, BakeryDataFilterParams};
use features_bakery_entities::baker::{BakerForCreateDto, ModelOptionDto};
use features_bakery_entities::bakery::ModelOptionDto as BakeryModelOptionDto;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"name": "NGHIA NGUYEN","bakery_id":1, "contact_details": {"field1": "Value 1", "field2": "Value 2"}}))]
pub struct BakerForCreateRequest {
    #[validate(length(
        min = 1,
        max = 16,
        code = "name_length",
        message = "the length of email must be between 1 and 50"
    ))]
    pub name: String,
    pub contact_details: Json,
    pub bakery_id: i32,
}

impl Into<BakerForCreateDto> for BakerForCreateRequest {
    fn into(self) -> BakerForCreateDto {
        BakerForCreateDto {
            name: self.name,
            contact_details: self.contact_details,
            bakery_id: self.bakery_id,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct BakerData {
    id: Option<i32>,
    bakery_id: Option<i32>,
    bakery: Option<BakeryData>,
    name: Option<String>,
    contact_details: Option<Json>,
}

impl Into<BakerData> for ModelOptionDto {
    fn into(self) -> BakerData {
        BakerData {
            name: self.name,
            contact_details: self.contact_details,
            id: self.id,
            bakery_id: self.bakery_id,
            ..Default::default()
        }
    }
}

impl From<(ModelOptionDto, BakeryModelOptionDto)> for BakerData {
    fn from((baker, bakery): (ModelOptionDto, BakeryModelOptionDto)) -> Self {
        BakerData {
            id: baker.id,
            name: baker.name,
            contact_details: baker.contact_details,
            bakery_id: baker.bakery_id,
            bakery: Some(bakery.into()),
            ..Default::default()
        }
    }
}
