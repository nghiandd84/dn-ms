use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Deserializer};
use tracing::debug;

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    debug!("Deserialize datetime: {}", value);
    let parsed = chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M"))
        .map_err(serde::de::Error::custom)?;
    Ok(parsed)
}

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(v) if v.is_empty() => Ok(None),
        Some(v) => {
            debug!("Deserialize optional datetime: {}", v);
            let parsed = chrono::NaiveDateTime::parse_from_str(&v, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(&v, "%Y-%m-%dT%H:%M"))
                .map_err(serde::de::Error::custom)?;
            Ok(Some(parsed))
        }
    }
}
