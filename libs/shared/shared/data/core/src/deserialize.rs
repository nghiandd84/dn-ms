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
        .map_err(serde::de::Error::custom)?;
    Ok(parsed)
}
