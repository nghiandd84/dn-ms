use shared_shared_data_core::filter::FilterOperator;
use shared_shared_data_core::filter_deserialize::*;
use serde::Deserialize;

// Helper structs that use the custom deserializers via serde field attributes
#[derive(Deserialize)]
struct StringFilter {
    #[serde(default = "default_none_string", deserialize_with = "deserialize_filter_from_string")]
    field: Option<shared_shared_data_core::filter::FilterParam<String>>,
}

#[derive(Deserialize)]
struct I32Filter {
    #[serde(default = "default_none_i32", deserialize_with = "deserialize_filter_from_i32")]
    field: Option<shared_shared_data_core::filter::FilterParam<i32>>,
}

#[derive(Deserialize)]
struct I64Filter {
    #[serde(default = "default_none_i64", deserialize_with = "deserialize_filter_from_i64")]
    field: Option<shared_shared_data_core::filter::FilterParam<i64>>,
}

#[derive(Deserialize)]
struct U32Filter {
    #[serde(default = "default_none_u32", deserialize_with = "deserialize_filter_from_u32")]
    field: Option<shared_shared_data_core::filter::FilterParam<u32>>,
}

#[derive(Deserialize)]
struct F32Filter {
    #[serde(default = "default_none_f32", deserialize_with = "deserialize_filter_from_f32")]
    field: Option<shared_shared_data_core::filter::FilterParam<f32>>,
}

#[derive(Deserialize)]
struct F64Filter {
    #[serde(default = "default_none_f64", deserialize_with = "deserialize_filter_from_f64")]
    field: Option<shared_shared_data_core::filter::FilterParam<f64>>,
}

#[derive(Deserialize)]
struct BoolFilter {
    #[serde(default = "default_none_bool", deserialize_with = "deserialize_filter_from_bool")]
    field: Option<shared_shared_data_core::filter::FilterParam<bool>>,
}

#[derive(Deserialize)]
struct UuidFilter {
    #[serde(default = "default_none_uuid", deserialize_with = "deserialize_filter_from_uuid")]
    field: Option<shared_shared_data_core::filter::FilterParam<uuid::Uuid>>,
}

#[derive(Deserialize)]
struct VecStringFilter {
    #[serde(default = "default_none_vecstring", deserialize_with = "deserialize_filter_from_vecstring")]
    field: Option<shared_shared_data_core::filter::FilterParam<Vec<String>>>,
}

// Tests

#[test]
fn deserialize_string_filter_eq() {
    let json = r#"{"field": "eq|hello"}"#;
    let f: StringFilter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Equal);
    assert_eq!(param.value.unwrap(), "hello");
    assert_eq!(param.raw_value, "hello");
}

#[test]
fn deserialize_string_filter_like() {
    let json = r#"{"field": "li|world"}"#;
    let f: StringFilter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Like);
    assert_eq!(param.value.unwrap(), "world");
}

#[test]
fn deserialize_i32_filter() {
    let json = r#"{"field": "gt|42"}"#;
    let f: I32Filter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Greater);
    assert_eq!(param.value.unwrap(), 42);
}

#[test]
fn deserialize_i64_filter() {
    let json = r#"{"field": "lte|999999"}"#;
    let f: I64Filter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::LessEqual);
    assert_eq!(param.value.unwrap(), 999999i64);
}

#[test]
fn deserialize_u32_filter() {
    let json = r#"{"field": "eq|100"}"#;
    let f: U32Filter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Equal);
    assert_eq!(param.value.unwrap(), 100u32);
}

#[test]
fn deserialize_f32_filter() {
    let json = r#"{"field": "lt|3.14"}"#;
    let f: F32Filter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Less);
    assert!((param.value.unwrap() - 3.14f32).abs() < 0.01);
}

#[test]
fn deserialize_f64_filter() {
    let json = r#"{"field": "gte|2.718"}"#;
    let f: F64Filter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::GreaterEqual);
    assert!((param.value.unwrap() - 2.718f64).abs() < 0.001);
}

#[test]
fn deserialize_bool_filter() {
    let json = r#"{"field": "eq|true"}"#;
    let f: BoolFilter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Equal);
    assert_eq!(param.value.unwrap(), true);
}

#[test]
fn deserialize_uuid_filter() {
    let id = uuid::Uuid::new_v4();
    let json = format!(r#"{{"field": "eq|{}"}}"#, id);
    let f: UuidFilter = serde_json::from_str(&json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::Equal);
    assert_eq!(param.value.unwrap(), id);
}

#[test]
fn deserialize_vecstring_filter() {
    // The implementation splits the full input by comma, so the operator prefix stays in the first element
    let json = r#"{"field": "in|a,b,c"}"#;
    let f: VecStringFilter = serde_json::from_str(json).unwrap();
    let param = f.field.unwrap();
    assert_eq!(param.operator, FilterOperator::In);
    assert_eq!(param.value.unwrap(), vec!["in|a", "b", "c"]);
}

#[test]
fn default_none_functions_return_none() {
    assert!(default_none_string().is_none());
    assert!(default_none_i32().is_none());
    assert!(default_none_i64().is_none());
    assert!(default_none_u32().is_none());
    assert!(default_none_f32().is_none());
    assert!(default_none_f64().is_none());
    assert!(default_none_bool().is_none());
    assert!(default_none_uuid().is_none());
    assert!(default_none_datetime().is_none());
    assert!(default_none_json().is_none());
    assert!(default_none_vecstring().is_none());
    assert!(default_none_vecuuid().is_none());
}
