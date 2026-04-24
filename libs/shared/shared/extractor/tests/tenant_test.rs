use axum::extract::FromRequestParts;
use axum::http::{HeaderValue, Method, Request};
use shared_shared_extractor::TenantId;

async fn extract(baggage: Option<&str>) -> String {
    let mut builder = Request::builder().method(Method::GET).uri("/test");
    if let Some(val) = baggage {
        builder = builder.header("baggage", HeaderValue::from_str(val).unwrap());
    }
    let req = builder.body(()).unwrap();
    let mut parts = req.into_parts().0;
    let TenantId(id) = TenantId::from_request_parts(&mut parts, &()).await.unwrap();
    id
}

#[tokio::test]
async fn test_no_baggage_header() {
    assert_eq!(extract(None).await, "");
}

#[tokio::test]
async fn test_empty_baggage() {
    assert_eq!(extract(Some("")).await, "");
}

#[tokio::test]
async fn test_baggage_without_tenant_id() {
    assert_eq!(extract(Some("user_id=abc,client_id=xyz")).await, "");
}

#[tokio::test]
async fn test_tenant_id_only() {
    assert_eq!(extract(Some("tenant_id=t1")).await, "t1");
}

#[tokio::test]
async fn test_tenant_id_with_other_fields() {
    assert_eq!(
        extract(Some("user_id=u1,tenant_id=tenant-abc,client_id=c1")).await,
        "tenant-abc"
    );
}

#[tokio::test]
async fn test_tenant_id_with_spaces() {
    assert_eq!(
        extract(Some("user_id=u1, tenant_id = my-tenant , client_id=c1")).await,
        "my-tenant"
    );
}

#[tokio::test]
async fn test_tenant_id_empty_value() {
    assert_eq!(extract(Some("tenant_id=")).await, "");
}
