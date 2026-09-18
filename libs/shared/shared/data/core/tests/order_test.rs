use shared_shared_data_core::order::*;

#[test]
fn order_direction_deserialize_asc() {
    let dir: OrderDirection = serde_json::from_str("1").unwrap();
    matches!(dir, OrderDirection::Asc);
}

#[test]
fn order_direction_deserialize_desc() {
    let dir: OrderDirection = serde_json::from_str("-1").unwrap();
    matches!(dir, OrderDirection::Desc);
}

#[test]
fn order_direction_deserialize_invalid() {
    let result = serde_json::from_str::<OrderDirection>("0");
    assert!(result.is_err());
}

#[test]
fn order_default_has_none_fields() {
    let o = Order::default();
    assert!(o.order_name.is_none());
    assert!(o.order_direction.is_none());
}

#[test]
fn order_deserialize_with_values() {
    let json = r#"{"order_name": "created_at", "order_direction": -1}"#;
    let o: Order = serde_json::from_str(json).unwrap();
    assert_eq!(o.order_name.unwrap(), "created_at");
    assert!(matches!(o.order_direction.unwrap(), OrderDirection::Desc));
}

#[test]
fn order_deserialize_empty_defaults() {
    let json = r#"{}"#;
    let o: Order = serde_json::from_str(json).unwrap();
    assert!(o.order_name.is_none());
    assert!(o.order_direction.is_none());
}

#[test]
fn order_direction_deserialize_string_asc() {
    let dir: OrderDirection = serde_json::from_str("\"asc\"").unwrap();
    assert!(matches!(dir, OrderDirection::Asc));
}

#[test]
fn order_direction_deserialize_string_desc_case_insensitive() {
    let dir: OrderDirection = serde_json::from_str("\"DESC\"").unwrap();
    assert!(matches!(dir, OrderDirection::Desc));
}

#[test]
fn order_direction_deserialize_numeric_string() {
    let dir: OrderDirection = serde_json::from_str("\"-1\"").unwrap();
    assert!(matches!(dir, OrderDirection::Desc));
}

#[test]
fn order_direction_deserialize_string_invalid() {
    let result = serde_json::from_str::<OrderDirection>("\"sideways\"");
    assert!(result.is_err());
}
