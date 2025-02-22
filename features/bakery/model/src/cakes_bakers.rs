use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::cakes_bakers::{CakeBakerForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"cake_id":1,"baker_id":1}))]
pub struct CakeBakerForCreateRequest {
    pub cake_id: i32,
    pub baker_id: i32,
}

impl Into<CakeBakerForCreateDto> for CakeBakerForCreateRequest {
    fn into(self) -> CakeBakerForCreateDto {
        CakeBakerForCreateDto {
            cake_id: self.cake_id,
            baker_id: self.baker_id,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct CakeBakerData {
    cake_id: Option<i32>,
    baker_id: Option<i32>,
}

impl Into<CakeBakerData> for ModelOptionDto {
    fn into(self) -> CakeBakerData {
        CakeBakerData {
            cake_id: self.cake_id,
            baker_id: self.baker_id,
            ..Default::default()
        }
    }
}
