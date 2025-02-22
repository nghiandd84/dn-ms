use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_macro::{ParamFilter, Response};

use features_bakery_entities::customer::{CustomerForCreateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[schema(example = json!({"name": "NGHIA NGUYEN","notes": "Notes about the customer"}))]
pub struct CustomerForCreateRequest {
    #[validate(length(
        min = 1,
        max = 16,
        code = "name_length",
        message = "the length of email must be between 1 and 50"
    ))]
    pub name: String,
    pub notes: Option<String>,
}

impl Into<CustomerForCreateDto> for CustomerForCreateRequest {
    fn into(self) -> CustomerForCreateDto {
        CustomerForCreateDto {
            name: self.name,
            notes: self.notes,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct CustomerData {
    id: Option<i32>,
    name: Option<String>,
    notes: Option<String>,
}

impl Into<CustomerData> for ModelOptionDto {
    fn into(self) -> CustomerData {
        CustomerData {
            id: self.id,
            name: self.name,
            notes: self.notes.unwrap(),
            ..Default::default()
        }
    }
}
