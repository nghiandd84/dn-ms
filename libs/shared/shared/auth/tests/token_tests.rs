use shared_shared_auth::{
    claim::UserAccessData,
    token::{
        create_access_token, create_refresh_token, decode_access_token, decode_refresh_token,
        get_access_token_cache_key, get_refresh_token_cache_key, insecured_decode_access_token,
        REFRESH_TOKEN_EXPIRATION, TOKEN_EXPIRATION, TOKEN_TYPE,
    },
};
use uuid::Uuid;

const SECRET: &str = "test-secret-key-for-unit-tests";

fn test_user_id() -> Uuid {
    Uuid::parse_str("066df7b0-dcd1-4e7c-94a1-9b5f68794ca7").unwrap()
}

fn test_client_id() -> Uuid {
    Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
}

fn test_accesses() -> Vec<UserAccessData> {
    vec![
        UserAccessData {
            role_name: "admin".to_string(),
            key: Some("key1".to_string()),
        },
        UserAccessData {
            role_name: "user".to_string(),
            key: None,
        },
    ]
}

// --- Constants ---

#[test]
fn test_constants() {
    assert_eq!(TOKEN_TYPE, "Bearer");
    assert_eq!(TOKEN_EXPIRATION, 86400);
    assert_eq!(REFRESH_TOKEN_EXPIRATION, 2592000);
}

// --- Cache key helpers ---

#[test]
fn test_access_token_cache_key() {
    let uid = test_user_id();
    assert_eq!(
        get_access_token_cache_key(uid),
        format!("{}:A", uid)
    );
}

#[test]
fn test_refresh_token_cache_key() {
    let uid = test_user_id();
    assert_eq!(
        get_refresh_token_cache_key(uid),
        format!("{}:R", uid)
    );
}

// --- create_access_token ---

#[test]
fn test_create_access_token_returns_token_and_jti() {
    let (token, jti) = create_access_token(
        test_user_id(),
        test_client_id(),
        SECRET,
        test_accesses(),
    )
    .expect("Should create access token");

    assert!(!token.is_empty());
    assert!(!jti.is_nil());
}

// --- create_refresh_token ---

#[test]
fn test_create_refresh_token_returns_token_and_jti() {
    let token_id = Uuid::new_v4();
    let (token, jti) = create_refresh_token(
        test_user_id(),
        test_client_id(),
        SECRET,
        token_id,
    )
    .expect("Should create refresh token");

    assert!(!token.is_empty());
    assert!(!jti.is_nil());
}

// --- decode_access_token (round-trip) ---

#[test]
fn test_decode_access_token_round_trip() {
    let user_id = test_user_id();
    let client_id = test_client_id();
    let accesses = test_accesses();

    let (token, _jti) =
        create_access_token(user_id, client_id, SECRET, accesses.clone()).unwrap();

    let decoded = decode_access_token(&token, SECRET).expect("Should decode access token");

    assert_eq!(decoded.user_id, user_id);
    assert_eq!(decoded.client_id, client_id);
    assert_eq!(decoded.accesses, accesses);
}

#[test]
fn test_decode_access_token_wrong_secret_fails() {
    let (token, _) =
        create_access_token(test_user_id(), test_client_id(), SECRET, test_accesses()).unwrap();

    let result = decode_access_token(&token, "wrong-secret");
    assert!(result.is_err());
}

#[test]
fn test_decode_access_token_invalid_token_fails() {
    let result = decode_access_token("not.a.valid.jwt", SECRET);
    assert!(result.is_err());
}

// --- insecured_decode_access_token ---

#[test]
fn test_insecured_decode_access_token_round_trip() {
    let user_id = test_user_id();
    let client_id = test_client_id();
    let accesses = test_accesses();

    let (token, _) =
        create_access_token(user_id, client_id, SECRET, accesses.clone()).unwrap();

    let decoded =
        insecured_decode_access_token(&token).expect("Should decode without secret");

    assert_eq!(decoded.user_id, user_id);
    assert_eq!(decoded.client_id, client_id);
    assert_eq!(decoded.accesses, accesses);
}

#[test]
fn test_insecured_decode_access_token_invalid_token_fails() {
    let result = insecured_decode_access_token("garbage");
    assert!(result.is_err());
}

#[test]
fn test_insecured_decode_with_refresh_token_fails() {
    let (refresh_token, _) =
        create_refresh_token(test_user_id(), test_client_id(), SECRET, Uuid::new_v4()).unwrap();

    let result = insecured_decode_access_token(&refresh_token);
    assert!(result.is_err(), "Refresh token should not decode as access token");
}

// --- decode_refresh_token (round-trip) ---

#[test]
fn test_decode_refresh_token_round_trip() {
    let user_id = test_user_id();
    let client_id = test_client_id();
    let token_id = Uuid::new_v4();

    let (token, jti) =
        create_refresh_token(user_id, client_id, SECRET, token_id).unwrap();

    let (decoded, decoded_jti) =
        decode_refresh_token(&token, SECRET).expect("Should decode refresh token");

    assert_eq!(decoded.user_id, user_id);
    assert_eq!(decoded.client_id, client_id);
    assert_eq!(decoded.token_id, token_id);
    assert_eq!(decoded_jti, jti);
}

#[test]
fn test_decode_refresh_token_wrong_secret_fails() {
    let (token, _) =
        create_refresh_token(test_user_id(), test_client_id(), SECRET, Uuid::new_v4()).unwrap();

    let result = decode_refresh_token(&token, "wrong-secret");
    assert!(result.is_err());
}

#[test]
fn test_decode_refresh_token_with_access_token_fails() {
    let (access_token, _) =
        create_access_token(test_user_id(), test_client_id(), SECRET, test_accesses()).unwrap();

    let result = decode_refresh_token(&access_token, SECRET);
    assert!(result.is_err(), "Access token should not decode as refresh token");
}
