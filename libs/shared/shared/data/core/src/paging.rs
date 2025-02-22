use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use shared_shared_macro::ResponseGeneric;

#[derive(ResponseGeneric, Serialize, Debug, Clone, ToSchema)]
pub struct QueryResult<T> {
    pub total_page: u64,
    pub result: Vec<T>,
}
#[derive(Deserialize, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[param(value_type = Option<u64>)]
    pub page: Option<u64>,
    #[serde(default = "default_page_size")]
    #[param(value_type = Option<u64>)]
    pub page_size: Option<u64>,
}

fn default_page() -> Option<u64> {
    Some(1)
}

fn default_page_size() -> Option<u64> {
    Some(10)
}

impl Pagination {
    pub fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

