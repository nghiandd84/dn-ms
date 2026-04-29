use uuid::Uuid;

use shared_shared_macro::RemoteService;

use features_payments_core_model::payment::{
    PaymentData, PaymentForCreateRequest, PaymentForUpdateRequest,
};

#[derive(Debug, RemoteService)]
#[remote(name(payment_core_service))]
pub struct PaymentRemoteService {}

impl PaymentRemoteService {
    fn headers_with_baggage(baggage: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("baggage".to_string(), baggage.to_string());
        headers
    }

    pub async fn create_payment(
        baggage: &str,
        req: PaymentForCreateRequest,
    ) -> Result<Uuid, String> {
        let endpoint = std::env::var("PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT")
            .expect("PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT must be set");

        let body = serde_json::to_value(&req).map_err(|e| e.to_string())?;
        let data = Self::call_api(
            endpoint,
            Method::POST,
            Some(body),
            Self::headers_with_baggage(baggage),
        )
        .await?;

        let id = data
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or("Failed to parse payment id from response")?;
        Ok(id)
    }

    pub async fn get_payment_by_id(baggage: &str, payment_id: Uuid) -> Result<PaymentData, String> {
        let endpoint = std::env::var("PAYMENT_CORE_ENDPOINT_GET_PAYMENT")
            .expect("PAYMENT_CORE_ENDPOINT_GET_PAYMENT must be set");

        let url = format!("{}/{}", endpoint, payment_id);
        let data =
            Self::call_api(url, Method::GET, None, Self::headers_with_baggage(baggage)).await?;
        serde_json::from_value::<PaymentData>(data).map_err(|e| e.to_string())
    }

    pub async fn update_payment(
        baggage: &str,
        payment_id: Uuid,
        req: PaymentForUpdateRequest,
    ) -> Result<bool, String> {
        let endpoint = std::env::var("PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT")
            .expect("PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT must be set");

        let url = format!("{}/{}", endpoint, payment_id);
        let body = serde_json::to_value(&req).map_err(|e| e.to_string())?;
        let _data = Self::call_api(
            url,
            Method::PATCH,
            Some(body),
            Self::headers_with_baggage(baggage),
        )
        .await?;
        Ok(true)
    }
}
