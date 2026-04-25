use shared_shared_data_core::paging::*;

#[test]
fn pagination_default_values() {
    let p = Pagination::default();
    assert_eq!(p.page, Some(1));
    assert_eq!(p.page_size, Some(10));
}

#[test]
fn pagination_new() {
    let p = Pagination::new(3, 15);
    assert_eq!(p.page, Some(3));
    assert_eq!(p.page_size, Some(15));
}

#[test]
fn pagination_deserialize_defaults() {
    let json = r#"{}"#;
    let p: Pagination = serde_json::from_str(json).unwrap();
    assert_eq!(p.page, Some(1));
    assert_eq!(p.page_size, Some(10));
}

#[test]
fn pagination_deserialize_custom() {
    let json = r#"{"page": 5, "page_size": 20}"#;
    let p: Pagination = serde_json::from_str(json).unwrap();
    assert_eq!(p.page, Some(5));
    assert_eq!(p.page_size, Some(20));
}

#[test]
fn pagination_page_size_exceeds_max() {
    let json = r#"{"page": 1, "page_size": 21}"#;
    let result = serde_json::from_str::<Pagination>(json);
    assert!(result.is_err());
}

#[test]
fn query_result_serialize() {
    let qr = QueryResult {
        total_page: 5,
        result: vec![1, 2, 3],
    };
    let json = serde_json::to_value(&qr).unwrap();
    assert_eq!(json["total_page"], 5);
    assert_eq!(json["result"], serde_json::json!([1, 2, 3]));
}

#[test]
fn query_result_deserialize() {
    let json = r#"{"total_page": 2, "result": ["a", "b"]}"#;
    let qr: QueryResult<String> = serde_json::from_str(json).unwrap();
    assert_eq!(qr.total_page, 2);
    assert_eq!(qr.result, vec!["a", "b"]);
}
