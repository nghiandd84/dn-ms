use uuid::Uuid;

use shared_shared_macro::RemoteService;

use features_payments_core_model::payment_attempt::PaymentAttemptForCreateRequest;

#[derive(Debug, RemoteService)]
#[remote(name(payment_core_service))]
pub struct PaymentAttemptRemoteService {}

impl PaymentAttemptRemoteService {
    pub async fn create_payment_attempt(
        baggage: &str,
        req: PaymentAttemptForCreateRequest,
    ) -> Result<Uuid, String> {
        let endpoint = std::env::var("PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT")
            .expect("PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT must be set");

        let mut headers = HashMap::new();
        headers.insert("baggage".to_string(), baggage.to_string());

        let body = serde_json::to_value(&req).map_err(|e| e.to_string())?;
        let data = Self::call_api(endpoint, Method::POST, Some(body), headers).await?;

        let id = data
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or("Failed to parse payment attempt id from response")?;
        Ok(id)
    }
}
