use shared_shared_auth::claim::{AccessTokenStruct, UserAccessData};
use uuid::Uuid;

#[test]
fn test_from_string_empty_user_id() {
    let client_id = Uuid::new_v4();
    let s = format!("accesses=admin*key1,user_id=,client_id={}", client_id);
    let ats = AccessTokenStruct::from_string(&s);
    assert!(ats.is_none(), "Should return None if user_id is empty");
}

#[test]
fn test_from_string_empty_client_id() {
    let user_id = Uuid::new_v4();
    let s = format!("accesses=admin*key1,user_id={},client_id=", user_id);
    let ats = AccessTokenStruct::from_string(&s);
    assert!(ats.is_none(), "Should return None if client_id is empty");
}

#[test]
fn test_from_string_invalid_user_id() {
    let client_id = Uuid::new_v4();
    let s = format!(
        "accesses=admin*key1,user_id=not-a-uuid,client_id={}",
        client_id
    );
    let ats = AccessTokenStruct::from_string(&s);
    assert!(
        ats.is_none(),
        "Should return None if user_id is not a valid Uuid"
    );
}

#[test]
fn test_from_string_invalid_client_id() {
    let user_id = Uuid::new_v4();
    let s = format!(
        "accesses=admin*key1,user_id={},client_id=not-a-uuid",
        user_id
    );
    let ats = AccessTokenStruct::from_string(&s);
    assert!(
        ats.is_none(),
        "Should return None if client_id is not a valid Uuid"
    );
}

#[test]
fn test_access_token_struct_to_baggage() {
    let access = UserAccessData {
        role_name: "admin".to_string(),
        key: Some("key1".to_string()),
    };
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let ats = AccessTokenStruct {
        user_id,
        client_id,
        accesses: vec![access.clone()],
    };
    let baggage = ats.to_baggage();

    assert_eq!(
        baggage.get("user_id").map(|s| s.as_str()),
        Some(user_id.to_string().as_str())
    );
    assert_eq!(
        baggage.get("client_id").map(|s| s.as_str()),
        Some(client_id.to_string().as_str())
    );
    assert_eq!(
        baggage.get("accesses").map(|s| s.as_str()),
        Some(access.to_string().as_str())
    );
}

#[test]
fn test_access_token_struct_from_string() {
    let user_id = Uuid::new_v4();
    let client_id = Uuid::new_v4();
    let access_one = UserAccessData {
        role_name: "admin".to_string(),
        key: Some("key1".to_string()),
    };
    let access_two = UserAccessData {
        role_name: "user".to_string(),
        key: None,
    };
    let accesses: Vec<UserAccessData> = vec![access_one.clone(), access_two.clone()];
    let access_str = accesses
        .iter()
        .map(|access| access.to_string())
        .collect::<Vec<_>>()
        .join("|");

    let s = format!(
        "accesses={},user_id={},client_id={}",
        access_str, user_id, client_id
    );
    let ats = AccessTokenStruct::from_string(&s).expect("Should parse");
    assert_eq!(ats.user_id, user_id);
    assert_eq!(ats.client_id, client_id);
    assert_eq!(ats.accesses[0], access_one);
    assert_eq!(ats.accesses[1], access_two);
}

#[test]
fn test_access_token_struct_access_to_string() {
    let access1 = UserAccessData {
        role_name: "admin".to_string(),
        key: Some("key1".to_string()),
    };
    let access2 = UserAccessData {
        role_name: "user".to_string(),
        key: None,
    };
    let ats = AccessTokenStruct {
        user_id: Uuid::new_v4(),
        client_id: Uuid::new_v4(),
        accesses: vec![access1.clone(), access2.clone()],
    };
    let result = ats.access_to_string();
    assert_eq!(result, "admin*key1|user*");
}
