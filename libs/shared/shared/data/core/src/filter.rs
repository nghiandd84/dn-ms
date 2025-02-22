use chrono::NaiveDateTime as DateTime;
use serde::Deserialize;
use serde_json::Value as Json;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    Like,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    In,
    NotIn,
}
impl<'de> Deserialize<'de> for FilterOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        debug!("Filter Oerator Deserialize {}", value);

        match value.as_str() {
            "eq" => Ok(FilterOperator::Equal),
            "ne" => Ok(FilterOperator::NotEqual),
            "li" => Ok(FilterOperator::Like),
            "lt" => Ok(FilterOperator::Less),
            "lte" => Ok(FilterOperator::LessEqual),
            "gt" => Ok(FilterOperator::Greater),
            "gte" => Ok(FilterOperator::GreaterEqual),
            "in" => Ok(FilterOperator::In),
            "nin" => Ok(FilterOperator::NotIn),
            _ => Err(serde::de::Error::custom("Invalid filter operator value")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FilterEnum {
    String(FilterParam<String>),
    Bool(FilterParam<bool>),
    Json(FilterParam<Json>),
    I32(FilterParam<i32>),
    I8(FilterParam<i8>),
    U32(FilterParam<u32>),
    U64(FilterParam<u64>),
    F32(FilterParam<f32>),
    F64(FilterParam<f64>),
    Uuid(FilterParam<Uuid>),
    DateTime(FilterParam<DateTime>),
}

impl FilterEnum {
    pub fn get_name(self: &Self) -> String {
        match self {
            FilterEnum::String(param) => param.name.clone(),
            FilterEnum::Json(param) => param.name.clone(),
            FilterEnum::Bool(param) => param.name.clone(),
            FilterEnum::I8(param) => param.name.clone(),
            FilterEnum::I32(param) => param.name.clone(),
            FilterEnum::U32(param) => param.name.clone(),
            FilterEnum::U64(param) => param.name.clone(),
            FilterEnum::Uuid(param) => param.name.clone(),
            FilterEnum::F32(param) => param.name.clone(),
            FilterEnum::F64(param) => param.name.clone(),
            FilterEnum::DateTime(param) => param.name.clone(),
        }
    }

    pub fn add_name_prefix(self: &mut Self, prefix: &str) {
        let prefix = format!("{}.{}", prefix, self.get_name());

        match self {
            FilterEnum::String(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Bool(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Json(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::I32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::I8(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::U32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::U64(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::F32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::F64(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Uuid(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::DateTime(ref mut param) => {
                param.name = prefix.clone();
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilterParam<T> {
    pub name: String,
    pub value: Option<T>,
    pub raw_value: String,
    pub operator: FilterOperator,
}
