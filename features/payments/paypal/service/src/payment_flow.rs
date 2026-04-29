use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde_json::json;
use tracing::{debug, error};

use shared_shared_data_error::app::{remote_err, AppError};

use features_payments_core_model::payment::{PaymentForCreateRequest, PaymentForUpdateRequest};
use features_payments_core_model::payment_attempt::PaymentAttemptForCreateRequest;
use features_payments_core_remote::{PaymentAttemptRemoteService, PaymentRemoteService};

use features_payments_paypal_model::payment_flow::{
    CapturePaymentRequest, CapturePaymentResponse, InitiatePaymentRequest,
    InitiatePaymentResponse, RefundPaymentRequest, RefundPaymentResponse,
};
use features_payments_paypal_model::paypal_order::{
    PaypalOrderForCreateRequest, PaypalOrderForUpdateRequest,
};
use features_payments_paypal_model::paypal_refund::PaypalRefundForCreateRequest;
use features_payments_paypal_model::paypal_webhook_event::PaypalWebhookEventForCreateRequest;
use features_payments_paypal_model::state::PaymentsPaypalAppState;

use crate::{PaypalOrderService, PaypalRefundService, PaypalWebhookEventService};

pub struct PaymentFlowService;

impl PaymentFlowService {
    /// Get an OAuth2 access token from PayPal
    async fn get_access_token(state: &PaymentsPaypalAppState) -> Result<String, AppError> {
        let url = format!("{}/v1/oauth2/token", state.api_base);
        let credentials = BASE64.encode(format!("{}:{}", state.client_id, state.client_secret));

        let resp = state
            .http_client
            .post(&url)
            .header("Authorization", format!("Basic {}", credentials))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("PayPal auth request failed: {e}")))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("PayPal auth response parse failed: {e}")))?;

        body.get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Internal("No access_token in PayPal response".to_string()))
    }

    /// Convert amount from minor units (cents) to PayPal decimal string
    fn format_amount(amount: i64) -> String {
        format!("{}.{:02}", amount / 100, amount % 100)
    }

    pub async fn initiate_payment(
        state: &PaymentsPaypalAppState,
        baggage: &str,
        req: InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse, AppError> {
        debug!(
            "Initiating PayPal payment for user_id: {}, amount: {}, currency: {}",
            req.user_id, req.amount, req.currency
        );

        // 1. Create core payment via remote
        let payment_id = PaymentRemoteService::create_payment(
            baggage,
            PaymentForCreateRequest {
                user_id: req.user_id,
                amount: req.amount,
                currency: req.currency.clone(),
                provider_name: "paypal".to_string(),
                idempotency_key: req.idempotency_key.clone(),
                transaction_id: req.idempotency_key.clone(),
                gateway_transaction_id: "N/A".to_string(),
                metadata: req.metadata.clone(),
            },
        )
        .await
        .map_err(remote_err)?;

        // 2. Call PayPal Create Order API
        let access_token = Self::get_access_token(state).await?;
        let order_body = json!({
            "intent": "CAPTURE",
            "purchase_units": [{
                "amount": {
                    "currency_code": req.currency.to_uppercase(),
                    "value": Self::format_amount(req.amount)
                }
            }]
        });

        let resp = state
            .http_client
            .post(&format!("{}/v2/checkout/orders", state.api_base))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("PayPal-Request-Id", &req.idempotency_key)
            .json(&order_body)
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                error!("PayPal CreateOrder failed: {e}");
                let _ = PaymentRemoteService::update_payment(
                    baggage,
                    payment_id,
                    PaymentForUpdateRequest {
                        status: Some("failed".to_string()),
                        gateway_transaction_id: None,
                    },
                )
                .await;
                let _ = PaymentAttemptRemoteService::create_payment_attempt(
                    baggage,
                    PaymentAttemptForCreateRequest {
                        payment_id,
                        provider: "paypal".to_string(),
                        raw_request: order_body,
                        raw_response: json!(null),
                        success: false,
                        error_message: e.to_string(),
                    },
                )
                .await;
                return Err(AppError::Internal(
                    "Failed to create PayPal order".to_string(),
                ));
            }
        };

        let status_code = resp.status().as_u16();
        let resp_body: serde_json::Value = resp.json().await.unwrap_or_default();

        if status_code >= 400 {
            error!("PayPal CreateOrder returned {}: {:?}", status_code, resp_body);
            let _ = PaymentRemoteService::update_payment(
                baggage,
                payment_id,
                PaymentForUpdateRequest {
                    status: Some("failed".to_string()),
                    gateway_transaction_id: None,
                },
            )
            .await;
            let _ = PaymentAttemptRemoteService::create_payment_attempt(
                baggage,
                PaymentAttemptForCreateRequest {
                    payment_id,
                    provider: "paypal".to_string(),
                    raw_request: order_body,
                    raw_response: resp_body,
                    success: false,
                    error_message: format!("PayPal returned status {}", status_code),
                },
            )
            .await;
            return Err(AppError::Internal(
                "Failed to create PayPal order".to_string(),
            ));
        }

        let paypal_order_id = resp_body
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let approval_url = resp_body
            .get("links")
            .and_then(|links| links.as_array())
            .and_then(|links| {
                links
                    .iter()
                    .find(|l| l.get("rel").and_then(|r| r.as_str()) == Some("approve"))
            })
            .and_then(|l| l.get("href"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let order_status = resp_body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("CREATED")
            .to_string();

        // 3. Update core payment with gateway_transaction_id
        if let Err(e) = PaymentRemoteService::update_payment(
            baggage,
            payment_id,
            PaymentForUpdateRequest {
                status: Some("processing".to_string()),
                gateway_transaction_id: Some(paypal_order_id.clone()),
            },
        )
        .await
        {
            error!("Failed to update core payment {payment_id} with order {paypal_order_id}: {e}");
        }

        // 4. Persist paypal order record
        if let Err(e) = PaypalOrderService::create_order(PaypalOrderForCreateRequest {
            payment_id,
            paypal_order_id: paypal_order_id.clone(),
            amount: req.amount,
            currency: req.currency.clone(),
            status: order_status,
            approval_url: Some(approval_url.clone()),
            capture_id: None,
            metadata: req.metadata,
        })
        .await
        {
            error!("Failed to persist paypal order record for {paypal_order_id}: {e}");
        }

        // 5. Log attempt
        let _ = PaymentAttemptRemoteService::create_payment_attempt(
            baggage,
            PaymentAttemptForCreateRequest {
                payment_id,
                provider: "paypal".to_string(),
                raw_request: order_body,
                raw_response: resp_body,
                success: true,
                error_message: String::new(),
            },
        )
        .await;

        Ok(InitiatePaymentResponse {
            payment_id,
            paypal_order_id,
            approval_url,
        })
    }

    pub async fn capture_payment(
        state: &PaymentsPaypalAppState,
        baggage: &str,
        req: CapturePaymentRequest,
    ) -> Result<CapturePaymentResponse, AppError> {
        debug!("Capturing PayPal order: {}", req.paypal_order_id);

        let access_token = Self::get_access_token(state).await?;

        let resp = state
            .http_client
            .post(&format!(
                "{}/v2/checkout/orders/{}/capture",
                state.api_base, req.paypal_order_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("PayPal capture request failed: {e}")))?;

        let resp_body: serde_json::Value = resp.json().await.unwrap_or_default();

        let capture_status = resp_body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let capture_id = resp_body
            .get("purchase_units")
            .and_then(|pu| pu.get(0))
            .and_then(|pu| pu.get("payments"))
            .and_then(|p| p.get("captures"))
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        // Find local order record by paypal_order_id and update
        let order = Self::find_order_by_paypal_id(&req.paypal_order_id).await?;
        let order_record_id = order.id.ok_or_else(|| {
            AppError::Internal("Order record missing id".to_string())
        })?;
        let payment_id = order.payment_id.ok_or_else(|| {
            AppError::Internal("Order record missing payment_id".to_string())
        })?;

        // Update local order with capture info
        let _ = PaypalOrderService::update_order(
            order_record_id,
            PaypalOrderForUpdateRequest {
                status: Some(capture_status.clone()),
                capture_id: Some(capture_id.clone()),
                metadata: None,
            },
        )
        .await;

        // Update core payment status
        let new_status = if capture_status == "COMPLETED" {
            "succeeded"
        } else {
            "processing"
        };

        PaymentRemoteService::update_payment(
            baggage,
            payment_id,
            PaymentForUpdateRequest {
                status: Some(new_status.to_string()),
                gateway_transaction_id: None,
            },
        )
        .await
        .map_err(remote_err)?;

        Ok(CapturePaymentResponse {
            payment_id,
            capture_id,
            status: capture_status,
        })
    }

    pub async fn process_webhook(
        payload: &str,
        baggage: &str,
    ) -> Result<(), AppError> {
        let payload_json: serde_json::Value =
            serde_json::from_str(payload).unwrap_or_default();

        let event_id = payload_json
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let event_type = payload_json
            .get("event_type")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let resource = payload_json
            .get("resource")
            .cloned()
            .unwrap_or_default();

        // Persist webhook event
        let _ = PaypalWebhookEventService::create_webhook_event(
            PaypalWebhookEventForCreateRequest {
                paypal_event_id: event_id,
                event_type: event_type.clone(),
                event_data: payload_json.clone(),
                processed: Some(false),
                processing_error: None,
            },
        )
        .await;

        // Handle known event types
        match event_type.as_str() {
            "CHECKOUT.ORDER.APPROVED" => {
                // Order approved by buyer — ready for capture
                if let Some(order_id) = resource.get("id").and_then(|v| v.as_str()) {
                    if let Ok(order) = Self::find_order_by_paypal_id(order_id).await {
                        if let Some(record_id) = order.id {
                            let _ = PaypalOrderService::update_order(
                                record_id,
                                PaypalOrderForUpdateRequest {
                                    status: Some("APPROVED".to_string()),
                                    capture_id: None,
                                    metadata: None,
                                },
                            )
                            .await;
                        }
                    }
                }
            }
            "PAYMENT.CAPTURE.COMPLETED" => {
                let capture_id = resource.get("id").and_then(|v| v.as_str());
                let order_id = resource
                    .get("supplementary_data")
                    .and_then(|s| s.get("related_ids"))
                    .and_then(|r| r.get("order_id"))
                    .and_then(|v| v.as_str());

                if let Some(order_id) = order_id {
                    if let Ok(order) = Self::find_order_by_paypal_id(order_id).await {
                        if let Some(record_id) = order.id {
                            let _ = PaypalOrderService::update_order(
                                record_id,
                                PaypalOrderForUpdateRequest {
                                    status: Some("COMPLETED".to_string()),
                                    capture_id: capture_id.map(|s| s.to_string()),
                                    metadata: None,
                                },
                            )
                            .await;
                        }
                        if let Some(payment_id) = order.payment_id {
                            let _ = PaymentRemoteService::update_payment(
                                baggage,
                                payment_id,
                                PaymentForUpdateRequest {
                                    status: Some("succeeded".to_string()),
                                    gateway_transaction_id: None,
                                },
                            )
                            .await;
                        }
                    }
                }
            }
            "PAYMENT.CAPTURE.DENIED" => {
                let order_id = resource
                    .get("supplementary_data")
                    .and_then(|s| s.get("related_ids"))
                    .and_then(|r| r.get("order_id"))
                    .and_then(|v| v.as_str());

                if let Some(order_id) = order_id {
                    if let Ok(order) = Self::find_order_by_paypal_id(order_id).await {
                        if let Some(payment_id) = order.payment_id {
                            let _ = PaymentRemoteService::update_payment(
                                baggage,
                                payment_id,
                                PaymentForUpdateRequest {
                                    status: Some("failed".to_string()),
                                    gateway_transaction_id: None,
                                },
                            )
                            .await;
                        }
                    }
                }
            }
            "PAYMENT.CAPTURE.REFUNDED" => {
                debug!("Capture refunded event received");
            }
            _ => {
                debug!("Unhandled PayPal event type: {event_type}");
            }
        }

        Ok(())
    }

    pub async fn refund_payment(
        state: &PaymentsPaypalAppState,
        baggage: &str,
        req: RefundPaymentRequest,
    ) -> Result<RefundPaymentResponse, AppError> {
        let payment = PaymentRemoteService::get_payment_by_id(baggage, req.payment_id)
            .await
            .map_err(remote_err)?;

        let paypal_order_id = payment
            .gateway_transaction_id
            .ok_or_else(|| AppError::Internal("Payment has no PayPal order".to_string()))?;

        // Find local order to get capture_id
        let order = Self::find_order_by_paypal_id(&paypal_order_id).await?;
        let capture_id = order
            .capture_id
            .filter(|s| !s.is_empty())
            .ok_or_else(|| AppError::Internal("Order has no capture_id".to_string()))?;

        let access_token = Self::get_access_token(state).await?;
        let currency = payment.currency.unwrap_or_else(|| "USD".to_string());

        let refund_body = if let Some(amount) = req.amount {
            json!({
                "amount": {
                    "value": Self::format_amount(amount),
                    "currency_code": currency.to_uppercase()
                }
            })
        } else {
            json!({})
        };

        let resp = state
            .http_client
            .post(&format!(
                "{}/v2/payments/captures/{}/refund",
                state.api_base, capture_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&refund_body)
            .send()
            .await
            .map_err(|e| {
                error!("PayPal refund request failed: {e}");
                AppError::Internal("Failed to create PayPal refund".to_string())
            })?;

        let resp_body: serde_json::Value = resp.json().await.unwrap_or_default();

        let paypal_refund_id = resp_body
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let refund_status = resp_body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let refund_amount = req.amount.unwrap_or(payment.amount.unwrap_or(0));

        let refund_record_id =
            PaypalRefundService::create_refund(PaypalRefundForCreateRequest {
                payment_id: req.payment_id,
                paypal_refund_id: paypal_refund_id.clone(),
                paypal_capture_id: capture_id,
                amount: refund_amount,
                currency,
                status: refund_status.clone(),
                reason: req.reason,
                metadata: None,
            })
            .await?;

        PaymentRemoteService::update_payment(
            baggage,
            req.payment_id,
            PaymentForUpdateRequest {
                status: Some("refunded".to_string()),
                gateway_transaction_id: None,
            },
        )
        .await
        .map_err(remote_err)?;

        Ok(RefundPaymentResponse {
            refund_id: refund_record_id,
            paypal_refund_id,
            status: refund_status,
        })
    }

    async fn find_order_by_paypal_id(
        paypal_order_id: &str,
    ) -> Result<features_payments_paypal_model::paypal_order::PaypalOrderData, AppError> {
        use features_payments_paypal_entities::paypal_order::Column;
        use sea_orm::Iden;
        use shared_shared_data_core::{
            filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
            order::Order,
            paging::Pagination,
        };

        let param: FilterParam<String> = FilterParam {
            name: Column::PaypalOrderId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(paypal_order_id.to_string()),
            raw_value: paypal_order_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(param)].into();
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let result = PaypalOrderService::get_orders(&filters, &pagination, &order).await?;
        result
            .result
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal(format!("PayPal order not found: {paypal_order_id}")))
    }
}
