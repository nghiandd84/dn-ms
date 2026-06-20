use serde_json::json;
use shared_shared_macro::RemoteService;
use uuid::Uuid;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct ActiveCodeRemoteService {}

#[derive(Debug, serde::Deserialize)]
pub struct MarkAsSentResponse {
    pub marked: bool,
}

impl ActiveCodeRemoteService {
    pub async fn mark_as_sent(user_id: Uuid, code: String) -> Result<bool, String> {
        let endpoint = "/internal/active-codes/mark-sent".to_string();
        let body = json!({
            "user_id": user_id,
            "code": code,
        });

        let data = Self::call_api(endpoint, reqwest::Method::POST, Some(body), HashMap::new())
            .await?;

        let response: MarkAsSentResponse =
            serde_json::from_value(data).map_err(|e| e.to_string())?;
        debug!("mark_as_sent response: {:?}", response);
        Ok(response.marked)
    }
}
