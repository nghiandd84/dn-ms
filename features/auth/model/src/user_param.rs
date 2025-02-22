use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

use shared_shared_macro::ParamFilter;

#[derive(Serialize, Debug, Default, ParamFilter)]
pub struct UserData {
    first_name: Option<String>,
    id: Option<Uuid>,
    last_name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

// TODO implement macro to generate filter param
#[derive(Deserialize, Debug)]
pub struct UserXFilterParams {
    #[serde(
        default = "default_none_string",
        deserialize_with = "deserialize_filter_from_string"
    )]
    pub first_name: Option<FilterParam<String>>,
    #[serde(
        default = "default_none_string",
        deserialize_with = "deserialize_filter_from_string"
    )]
    pub last_name: Option<FilterParam<String>>,
    #[serde(
        default = "default_none_i32",
        deserialize_with = "deserialize_filter_from_i32"
    )]
    pub age: Option<FilterParam<i32>>,

    pub user: Option<UserDataFilterParams>,
}

impl UserXFilterParams {
    fn all_filters(self: &Self) -> Vec<FilterEnum> {
        let mut result: Vec<FilterEnum> = vec![];
        if self.first_name.is_some() {
            let mut filter = self.first_name.as_ref().unwrap().clone();
            filter.name = "first_name".to_owned();
            let filter_enum = FilterEnum::String(filter);
            result.push(filter_enum);
        }
        if self.user.is_some() {
            let mut filters = self.user.as_ref().unwrap().clone().all_filters();
            for f in filters.iter_mut() {
                f.add_name_prefix("user");
            }
            result.append(&mut filters);
        }
        result
    }
}
