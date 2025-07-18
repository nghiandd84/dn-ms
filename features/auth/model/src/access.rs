use serde::Serialize;
use utoipa::ToSchema;

use shared_shared_macro::{ParamFilter, Response};

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct UserAccessData {
    pub key: String,
    pub role_name: String,
}
