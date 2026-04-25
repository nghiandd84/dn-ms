use shared_shared_data_core::filter::*;
use uuid::Uuid;

#[test]
fn filter_operator_deserialize_all_variants() {
    let cases = vec![
        ("\"eq\"", FilterOperator::Equal),
        ("\"ne\"", FilterOperator::NotEqual),
        ("\"li\"", FilterOperator::Like),
        ("\"lt\"", FilterOperator::Less),
        ("\"lte\"", FilterOperator::LessEqual),
        ("\"gt\"", FilterOperator::Greater),
        ("\"gte\"", FilterOperator::GreaterEqual),
        ("\"in\"", FilterOperator::In),
        ("\"nin\"", FilterOperator::NotIn),
        ("\"sw\"", FilterOperator::StartWith),
    ];
    for (json, expected) in cases {
        let op: FilterOperator = serde_json::from_str(json).unwrap();
        assert_eq!(op, expected);
    }
}

#[test]
fn filter_operator_deserialize_invalid() {
    let result = serde_json::from_str::<FilterOperator>("\"bad\"");
    assert!(result.is_err());
}

#[test]
fn filter_enum_get_name() {
    let f = FilterEnum::String(FilterParam {
        name: "title".into(),
        value: Some("hello".into()),
        raw_value: "hello".into(),
        operator: FilterOperator::Equal,
    });
    assert_eq!(f.get_name(), "title");

    let f = FilterEnum::I32(FilterParam {
        name: "age".into(),
        value: Some(10),
        raw_value: "10".into(),
        operator: FilterOperator::Greater,
    });
    assert_eq!(f.get_name(), "age");

    let id = Uuid::new_v4();
    let f = FilterEnum::Uuid(FilterParam {
        name: "id".into(),
        value: Some(id),
        raw_value: id.to_string(),
        operator: FilterOperator::Equal,
    });
    assert_eq!(f.get_name(), "id");
}

#[test]
fn filter_enum_add_name_prefix() {
    let mut f = FilterEnum::String(FilterParam {
        name: "title".into(),
        value: Some("hello".into()),
        raw_value: "hello".into(),
        operator: FilterOperator::Equal,
    });
    f.add_name_prefix("entity");
    assert_eq!(f.get_name(), "entity.title");
}

#[test]
fn filter_enum_add_name_prefix_all_variants() {
    let mut variants: Vec<FilterEnum> = vec![
        FilterEnum::Bool(FilterParam { name: "a".into(), value: Some(true), raw_value: "true".into(), operator: FilterOperator::Equal }),
        FilterEnum::I8(FilterParam { name: "a".into(), value: Some(1), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::I64(FilterParam { name: "a".into(), value: Some(1), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::U32(FilterParam { name: "a".into(), value: Some(1), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::U64(FilterParam { name: "a".into(), value: Some(1), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::F32(FilterParam { name: "a".into(), value: Some(1.0), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::F64(FilterParam { name: "a".into(), value: Some(1.0), raw_value: "1".into(), operator: FilterOperator::Equal }),
        FilterEnum::Json(FilterParam { name: "a".into(), value: Some(serde_json::json!({})), raw_value: "{}".into(), operator: FilterOperator::Equal }),
        FilterEnum::DateTime(FilterParam { name: "a".into(), value: Some(chrono::NaiveDateTime::default()), raw_value: "".into(), operator: FilterOperator::Equal }),
        FilterEnum::VecString(FilterParam { name: "a".into(), value: Some(vec![]), raw_value: "".into(), operator: FilterOperator::Equal }),
        FilterEnum::VecUuid(FilterParam { name: "a".into(), value: Some(vec![]), raw_value: "".into(), operator: FilterOperator::Equal }),
    ];
    for f in &mut variants {
        f.add_name_prefix("pfx");
        assert_eq!(f.get_name(), "pfx.a");
    }
}

#[test]
fn convert_filter_param_to_query_string_all_operators() {
    let cases = vec![
        (FilterOperator::Equal, "eq"),
        (FilterOperator::NotEqual, "ne"),
        (FilterOperator::Like, "li"),
        (FilterOperator::Less, "lt"),
        (FilterOperator::LessEqual, "lte"),
        (FilterOperator::Greater, "gt"),
        (FilterOperator::GreaterEqual, "gte"),
        (FilterOperator::In, "in"),
        (FilterOperator::NotIn, "nin"),
        (FilterOperator::StartWith, "sw"),
    ];
    for (op, expected_str) in cases {
        let param = FilterParam::<String> {
            name: "field".into(),
            value: Some("val".into()),
            raw_value: "val".into(),
            operator: op,
        };
        let qs = convert_filter_param_to_query_string(&param);
        assert_eq!(qs, format!("field={}|val", expected_str));
    }
}
