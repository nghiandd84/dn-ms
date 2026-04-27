use opentelemetry::baggage::Baggage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_macro::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub dn_data: ClaimSubject,
    pub exp: u64, // Required: Expiration time
    pub iat: u64, // Optional: Issued at time
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // This tells Serde to try each variant until one matches
pub enum ClaimSubject {
    String(String),
    AccessToken(AccessTokenStruct),
    RefreshToken(RefreshTokenStruct),
}
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema, Response)]
pub struct AccessTokenStruct {
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub accesses: Vec<UserAccessData>,
}

impl AccessTokenStruct {
    pub fn to_baggage(&self) -> Baggage {
        let mut baggage = Baggage::new();
        let _ = baggage.insert("user_id", self.user_id.to_string());
        let _ = baggage.insert("client_id", self.client_id.to_string());
        let _ = baggage.insert("accesses", self.access_to_string());
        baggage
    }

    pub fn from_string(str: &str) -> Option<Self> {
        // Example input: accesses=BAKERY_SUPPORT*A_ACCESS_KEY|EMAIL_NOTIFICATION_SALE*B_ACCESS_KEY|SUPPORT*,user_id=066df7b0-dcd1-4e7c-94a1-9b5f68794ca7,client_id=123e4567-e89b-12d3-a456-426614174000
        let parts: HashMap<_, _> = str
            .split(',')
            .filter_map(|pair| {
                let mut kv = pair.splitn(2, '=');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    Some((key.trim(), value.trim()))
                } else {
                    None
                }
            })
            .collect();
        let user_id_str = parts.get("user_id")?;
        let client_id_str = parts.get("client_id")?;
        let user_id = Uuid::parse_str(user_id_str).ok()?;
        let client_id = Uuid::parse_str(client_id_str).ok()?;
        let accesses_str = parts.get("accesses")?;
        let accesses = accesses_str
            .split('|')
            .filter_map(|access| {
                let mut parts = access.splitn(2, '*');
                let role_name = parts.next()?.to_string();
                let key = parts
                    .next()
                    .map(|s| {
                        let r = s.to_string();
                        if r.is_empty() {
                            None
                        } else {
                            Some(r)
                        }
                    })
                    .unwrap();

                Some(UserAccessData { role_name, key })
            })
            .collect();

        Some(AccessTokenStruct {
            user_id: user_id,
            client_id: client_id,
            accesses: accesses,
        })
    }

    fn access_to_string(&self) -> String {
        let access_str = self
            .accesses
            .iter()
            .map(|access| access.to_string())
            .collect::<Vec<_>>()
            .join("|");
        access_str
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_from_string_empty_user_id() {
        let client_id = uuid::Uuid::new_v4();
        let s = format!("accesses=admin*key1,user_id=,client_id={}", client_id);
        let ats = AccessTokenStruct::from_string(&s);
        assert!(ats.is_none(), "Should return None if user_id is empty");
    }

    #[test]
    fn test_from_string_empty_client_id() {
        let user_id = uuid::Uuid::new_v4();
        let s = format!("accesses=admin*key1,user_id={},client_id=", user_id);
        let ats = AccessTokenStruct::from_string(&s);
        assert!(ats.is_none(), "Should return None if client_id is empty");
    }

    #[test]
    fn test_from_string_invalid_user_id() {
        let client_id = uuid::Uuid::new_v4();
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
        let user_id = uuid::Uuid::new_v4();
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

    use super::*;
    use uuid::Uuid;

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
        println!("Input string: {}", s);
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
        assert_eq!(result, format!("admin*key1|user*"));
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RefreshTokenStruct {
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub token_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, ToSchema)]
pub struct UserAccessData {
    pub role_name: String,
    pub key: Option<String>, // Optional: Access key
}

impl ToString for UserAccessData {
    fn to_string(&self) -> String {
        match &self.key {
            Some(key) => format!("{}*{}", self.role_name, key),
            None => format!("{}*", self.role_name),
        }
    }
}
