use chrono::NaiveDateTime;
use serde::Deserialize;
use shared_shared_data_core::deserialize::deserialize_datetime;

#[derive(Deserialize)]
struct Wrapper {
    #[serde(deserialize_with = "deserialize_datetime")]
    dt: NaiveDateTime,
}

#[test]
fn deserialize_datetime_valid() {
    let json = r#"{"dt": "2024-01-15T10:30:00"}"#;
    let w: Wrapper = serde_json::from_str(json).unwrap();
    assert_eq!(
        w.dt,
        NaiveDateTime::parse_from_str("2024-01-15T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap()
    );
}

#[test]
fn deserialize_datetime_invalid_format() {
    let json = r#"{"dt": "15/01/2024 10:30"}"#;
    let result = serde_json::from_str::<Wrapper>(json);
    assert!(result.is_err());
}

#[test]
fn deserialize_datetime_empty_string() {
    let json = r#"{"dt": ""}"#;
    let result = serde_json::from_str::<Wrapper>(json);
    assert!(result.is_err());
}
