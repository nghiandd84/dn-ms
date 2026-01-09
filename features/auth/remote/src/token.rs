use serde_json::json;

use shared_shared_auth::claim::{AccessTokenStruct, ClaimSubject};
use shared_shared_macro::RemoteService;
use shared_shared_middleware::RequestTracingMiddleware;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct TokenService {}

impl TokenService {
    pub async fn validate_token(token: String) -> Result<AccessTokenStruct, String> {
        let body = json!({
            "token": token
        });

        let verify_endpoint = std::env::var("AUTH_ENDPOINT_VERIFY_TOKEN")
            .expect("AUTH_ENDPOINT_VERIFY_TOKEN must be set");

        let res = Self::call_api(verify_endpoint, Method::POST, Some(body), HashMap::new()).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data = res.unwrap();
        debug!("Token validation response: {:?}", data);
        let claim_subject = serde_json::from_value::<ClaimSubject>(data.clone());
        if claim_subject.is_err() {
            let err_msg = format!(
                "Failed to parse claim subject: {}",
                claim_subject.err().unwrap()
            );
            return Err(err_msg);
        }
        let claim_subject = claim_subject.unwrap();
        debug!("Parsed claim subject: {:?}", claim_subject);
        let access_token = match claim_subject {
            ClaimSubject::AccessToken(access_token) => {
                debug!(
                    "Access token is valid for user_id: {}",
                    access_token.user_id
                );
                access_token
            }
            _ => {
                debug!("Claim subject is not an access token");
                return Err("Invalid token or claim subject".to_string());
            }
        };
        Ok(access_token)
    }
}
