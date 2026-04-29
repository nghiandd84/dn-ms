use axum::{
    extract::State,
    http::HeaderMap,
    routing::post,
    Router,
};
use tracing::{instrument, Level};

use features_payments_stripe_model::{
    payment_flow::{
        InitiatePaymentRequest, InitiatePaymentResponse, RefundPaymentRequest,
        RefundPaymentResponse,
    },
    state::{PaymentsStripeAppState, PaymentsStripeCacheState},
};
use features_payments_stripe_service::PaymentFlowService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::{Auth, PublicAccess};
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};

use crate::permission::CanCreatePaymentIntent;

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
    path = "/stripe/flow/initiate",
    tag = TAG,
    request_body = InitiatePaymentRequest,
    responses(
        (status = 200, description = "Payment initiated", body = InitiatePaymentResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn initiate_payment(
    _auth: Auth<CanCreatePaymentIntent>,
    State(app_state): State<AppState<PaymentsStripeAppState, PaymentsStripeCacheState>>,
    headers: HeaderMap,
    ValidJson(req): ValidJson<InitiatePaymentRequest>,
) -> Result<ResponseJson<InitiatePaymentResponse>> {
    let client = &app_state.get_state().unwrap().stripe_client;
    let baggage = extract_baggage(&headers);
    let response = PaymentFlowService::initiate_payment(client, &baggage, req).await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    path = "/stripe/flow/webhook",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook processed"),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn handle_webhook(
    _public: PublicAccess,
    headers: HeaderMap,
    payload: String,
) -> Result<ResponseJson<&'static str>> {
    let sig = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            shared_shared_data_error::app::AppError::Internal(
                "Missing stripe-signature header".to_string(),
            )
        })?;

    let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").map_err(|_| {
        shared_shared_data_error::app::AppError::Internal(
            "STRIPE_WEBHOOK_SECRET not configured".to_string(),
        )
    })?;

    let baggage = extract_baggage(&headers);
    PaymentFlowService::process_webhook(&payload, sig, &webhook_secret, &baggage).await?;
    Ok(ResponseJson("ok"))
}

#[utoipa::path(
    post,
    path = "/stripe/flow/refund",
    tag = TAG,
    request_body = RefundPaymentRequest,
    responses(
        (status = 200, description = "Refund created", body = RefundPaymentResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn refund_payment(
    _auth: Auth<CanCreatePaymentIntent>,
    State(app_state): State<AppState<PaymentsStripeAppState, PaymentsStripeCacheState>>,
    headers: HeaderMap,
    ValidJson(req): ValidJson<RefundPaymentRequest>,
) -> Result<ResponseJson<RefundPaymentResponse>> {
    let client = &app_state.get_state().unwrap().stripe_client;
    let baggage = extract_baggage(&headers);
    let response = PaymentFlowService::refund_payment(client, &baggage, req).await?;
    Ok(ResponseJson(response))
}

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .route("/stripe/flow/initiate", post(initiate_payment))
        .route("/stripe/flow/webhook", post(handle_webhook))
        .route("/stripe/flow/refund", post(refund_payment))
        .with_state(app_state.clone())
}
