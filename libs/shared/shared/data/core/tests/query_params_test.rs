use shared_shared_data_core::query_params::QueryParams;

#[test]
fn query_params_multiple_includes() {
    let json = r#"{"includes":"wallet,transaction,user"}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.includes(), vec!["wallet", "transaction", "user"]);
}

#[test]
fn query_params_single_include() {
    let json = r#"{"includes":"orders"}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.includes(), vec!["orders"]);
}

#[test]
fn query_params_empty_string() {
    let json = r#"{"includes":""}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.includes(), vec![""]);
}

#[test]
fn query_params_missing_includes() {
    let json = r#"{}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();
    assert!(params.includes().is_empty());
}
