use axum::extract::FromRequestParts;
use shared_shared_auth::{
    claim::UserAccessData,
    permission::{Auth, StatePermission, ADMIN, CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};
use shared_shared_data_error::auth::AuthError;
use std::marker::PhantomData;
use uuid::Uuid;

struct DummyResource;
impl ResourcePermission for DummyResource {
    const RESOURCE: &'static str = "dummy";
    const BIT: u32 = READ;
}

struct DummyState;
impl StatePermission for DummyState {
    fn get_permission_map(&self, _role_name: String, _resource_name: String) -> u32 {
        READ | CREATE | UPDATE | DELETE | ADMIN
    }
    async fn pull_permission(&self) -> Result<(), AuthError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_from_request_parts_with_baggage() {
    let state = DummyState;
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let access = UserAccessData {
        role_name: "dummy_role".to_string(),
        key: Some("key42".to_string()),
    };
    let accesses = vec![access.clone()];
    let access_str = accesses
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let baggage_value = format!(
        "accesses={},user_id={},client_id={}",
        access_str, user_id, client_id
    );

    let mut request = http::Request::builder();
    request = request.header("baggage", baggage_value.clone());
    let request = request.body(()).unwrap();
    let (mut parts, _body) = request.into_parts();

    let result = Auth::<DummyResource>::from_request_parts(&mut parts, &state).await;
    assert!(
        result.is_ok(),
        "Should extract Auth from valid baggage header"
    );
    let auth = result.unwrap();
    assert_eq!(auth.user_id(), Some(user_id));
    assert_eq!(auth.access_key(), Some("key42".to_string()));
    assert_eq!(auth.mask & READ, READ);
}

#[test]
fn test_permission_constants() {
    assert_eq!(READ, 1);
    assert_eq!(CREATE, 2);
    assert_eq!(UPDATE, 4);
    assert_eq!(DELETE, 8);
    assert_eq!(ADMIN, 16);
}

#[test]
fn test_auth_user_id_and_access_key() {
    let user_id = Uuid::new_v4();
    let auth: Auth<DummyResource> = Auth {
        mask: READ,
        phantom_r: PhantomData,
        user_id,
        access_key: Some("key123".to_string()),
    };
    assert_eq!(auth.user_id(), Some(user_id));
    assert_eq!(auth.access_key(), Some("key123".to_string()));
}
