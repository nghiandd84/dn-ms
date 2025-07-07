use std::fmt::Display;

use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

use crate::filter::FilterParam;

// String
pub fn default_none_string() -> Option<FilterParam<String>> {
    None
}

pub fn deserialize_filter_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<String, _>(&value, "".to_owned(), |s| format!("\"{}\"", s.to_string()))
        .map_err(serde::de::Error::custom)
}

// VecString
pub fn default_none_vecstring() -> Option<FilterParam<Vec<String>>> {
    None
}

pub fn deserialize_filter_from_vecstring<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<Vec<String>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    let r = parse_filter_param::<String, _>(&value, "".to_owned(), |s| {
        format!("\"{}\"", s.to_string())
    })
    .unwrap()
    .unwrap();
    let result: FilterParam<Vec<String>> = FilterParam {
        name: r.name,
        operator: r.operator,
        value: Some(value.split(',').map(|s| s.trim().to_string()).collect()),
        raw_value: r.raw_value,
    };
    Ok(Some(result))
}

// I32
pub fn default_none_i32() -> Option<FilterParam<i32>> {
    None
}

pub fn deserialize_filter_from_i32<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<i32, _>(&value, 0, |s| s.parse::<i32>().unwrap())
        .map_err(serde::de::Error::custom)
}

// U32
pub fn default_none_u32() -> Option<FilterParam<u32>> {
    None
}

pub fn deserialize_filter_from_u32<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<u32>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<u32, _>(&value, 0, |s| s.parse::<u32>().unwrap())
        .map_err(serde::de::Error::custom)
}

// F32
pub fn default_none_f64() -> Option<FilterParam<f64>> {
    None
}

pub fn deserialize_filter_from_f64<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<f64, _>(&value, 0f64, |s| s.parse::<f64>().unwrap())
        .map_err(serde::de::Error::custom)
}

// DateTime
pub fn default_none_datetime() -> Option<FilterParam<DateTime>> {
    None
}

pub fn deserialize_filter_from_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<DateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<DateTime, _>(&value, chrono::NaiveDateTime::MIN, |s| {
        s.parse::<DateTime>().unwrap()
    })
    .map_err(serde::de::Error::custom)
}

// Bool
pub fn default_none_bool() -> Option<FilterParam<bool>> {
    None
}

pub fn deserialize_filter_from_bool<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<bool>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_filter_param::<bool, _>(&value, false, |s| s.parse::<bool>().unwrap())
        .map_err(serde::de::Error::custom)
}

// Json
pub fn default_none_json() -> Option<FilterParam<Value>> {
    None
}

pub fn deserialize_filter_from_json<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<Value>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let default_value = json!({});

    let r = parse_filter_param::<Value, _>(&value, default_value, |s| {
        let value = serde_json::from_str(s).unwrap();
        value
    })
    .map_err(serde::de::Error::custom);
    r
}

// Uuid
pub fn default_none_uuid() -> Option<FilterParam<Uuid>> {
    None
}

pub fn deserialize_filter_from_uuid<'de, D>(
    deserializer: D,
) -> Result<Option<FilterParam<Uuid>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let (first_str, last_str) = value.split_once('|').unwrap_or(("", ""));
    let uuid_vaule = Uuid::parse_str(last_str).unwrap_or(Uuid::nil());

    let data = format!(
        r#"{{
            "name": "",
            "operator": "{}",
            "value": "{}",
            "raw_value": "{}"
        }}"#,
        first_str, uuid_vaule, last_str
    );

    match serde_json::from_str::<FilterParam<Uuid>>(data.as_str()) {
        Ok(filter_param) => Ok(Some(filter_param)),
        Err(err) => {
            debug!("Failed to parse filter parameter: {}", err);

            Ok(None)
        }
    }
}

// Internal function
fn parse_filter_param<T, U: FnOnce(&str) -> T>(
    value: &str,
    default_value: T,
    transform: U,
) -> Result<Option<FilterParam<T>>, serde_json::Error>
where
    T: serde::de::DeserializeOwned + Display + std::str::FromStr,
{
    let (first_str, last_str) = value.split_once('|').unwrap_or(("", ""));
    let parsed_value: T = last_str.parse().unwrap_or(default_value);
    let parsed_value = parsed_value.to_string();

    let data = format!(
        r#"{{
            "name": "",
            "operator": "{}",
            "value": {},
            "raw_value": "{}"
        }}"#,
        first_str,
        transform(parsed_value.as_str()),
        last_str
    );

    debug!("Data {}", data);

    match serde_json::from_str::<FilterParam<T>>(data.as_str()) {
        Ok(filter_param) => Ok(Some(filter_param)),
        Err(err) => {
            debug!("Failed to parse filter parameter: {}", err);
            Ok(None)
        }
    }
}
