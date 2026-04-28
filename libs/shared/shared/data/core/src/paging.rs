use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{IntoParams, ToSchema};

use shared_shared_macro::ResponseGeneric;

#[derive(ResponseGeneric, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct QueryResult<T> {
    pub total_page: u64,
    pub result: Vec<T>,
}

impl<T: serde::de::DeserializeOwned> QueryResult<T> {
    pub fn from_value(value: serde_json::Value) -> Result<Self, String> {
        serde_json::from_value(value).map_err(|e| {
            tracing::error!("Failed to deserialize QueryResult: {}", e);
            e.to_string()
        })
    }
}

fn deserialize_page_size<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<u64> = Option::deserialize(deserializer)?;
    if let Some(size) = value {
        if size > 20 {
            return Err(serde::de::Error::custom(
                "page_size must be less than or equal to 20",
            ));
        }
    }
    Ok(value)
}

#[derive(Deserialize, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[param(value_type = Option<u64>)]
    pub page: Option<u64>,
    #[serde(
        default = "default_page_size",
        deserialize_with = "deserialize_page_size"
    )]
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

    pub fn new(page: u64, page_size: u64) -> Self {
        Self {
            page: Some(page),
            page_size: Some(page_size),
        }
    }
}
