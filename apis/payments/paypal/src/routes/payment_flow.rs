use axum::{extract::State, http::HeaderMap, routing::post, Router};
use tracing::{instrument, Level};

use features_payments_paypal_model::{
    payment_flow::{
        CapturePaymentRequest, CapturePaymentResponse, InitiatePaymentRequest,
        InitiatePaymentResponse, RefundPaymentRequest, RefundPaymentResponse,
    },
    state::{PaymentsPaypalAppState, PaymentsPaypalCacheState},
};
use features_payments_paypal_service::PaymentFlowService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::{Auth, PublicAccess};
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};

use crate::permission::CanCreateOrder;

const TAG: &str = "payment_flow";

fn extract_baggage(headers: &HeaderMap) -> String {
    headers
        .get("baggage")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

#[utoipa::path(
    post,
    path = "/flow/initiate",
    tag = TAG,
    request_body = InitiatePaymentRequest,
    responses(
        (status = 200, description = "Payment initiated", body = InitiatePaymentResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn initiate_payment(
    _auth: Auth<CanCreateOrder>,
    State(app_state): State<AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>>,
    headers: HeaderMap,
    ValidJson(req): ValidJson<InitiatePaymentRequest>,
) -> Result<ResponseJson<InitiatePaymentResponse>> {
    let state = app_state.get_state().unwrap();
    let baggage = extract_baggage(&headers);
    let response = PaymentFlowService::initiate_payment(state, &baggage, req).await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    path = "/flow/capture",
    tag = TAG,
    request_body = CapturePaymentRequest,
    responses(
        (status = 200, description = "Payment captured", body = CapturePaymentResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn capture_payment(
    _auth: Auth<CanCreateOrder>,
    State(app_state): State<AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>>,
    headers: HeaderMap,
    ValidJson(req): ValidJson<CapturePaymentRequest>,
) -> Result<ResponseJson<CapturePaymentResponse>> {
    let state = app_state.get_state().unwrap();
    let baggage = extract_baggage(&headers);
    let response = PaymentFlowService::capture_payment(state, &baggage, req).await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    path = "/flow/webhook",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook processed"),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn handle_webhook(
    _public: PublicAccess,
    headers: HeaderMap,
    payload: String,
) -> Result<ResponseJson<&'static str>> {
    let baggage = extract_baggage(&headers);
    PaymentFlowService::process_webhook(&payload, &baggage).await?;
    Ok(ResponseJson("ok"))
}

#[utoipa::path(
    post,
    path = "/flow/refund",
    tag = TAG,
    request_body = RefundPaymentRequest,
    responses(
        (status = 200, description = "Refund created", body = RefundPaymentResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn refund_payment(
    _auth: Auth<CanCreateOrder>,
    State(app_state): State<AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>>,
    headers: HeaderMap,
    ValidJson(req): ValidJson<RefundPaymentRequest>,
) -> Result<ResponseJson<RefundPaymentResponse>> {
    let state = app_state.get_state().unwrap();
    let baggage = extract_baggage(&headers);
    let response = PaymentFlowService::refund_payment(state, &baggage, req).await?;
    Ok(ResponseJson(response))
}

pub fn routes(
    app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>,
) -> Router {
    Router::new()
        .route("/flow/initiate", post(initiate_payment))
        .route("/flow/capture", post(capture_payment))
        .route("/flow/webhook", post(handle_webhook))
        .route("/flow/refund", post(refund_payment))
        .with_state(app_state.clone())
}
