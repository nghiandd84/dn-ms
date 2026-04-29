use std::str::FromStr;

use serde_json::json;
use tracing::{debug, error};

use shared_shared_data_error::app::{remote_err, AppError};

use features_payments_core_model::payment::{PaymentForCreateRequest, PaymentForUpdateRequest};
use features_payments_core_model::payment_attempt::PaymentAttemptForCreateRequest;
use features_payments_core_remote::{PaymentAttemptRemoteService, PaymentRemoteService};

use features_payments_stripe_model::payment_flow::{
    InitiatePaymentRequest, InitiatePaymentResponse, RefundPaymentRequest, RefundPaymentResponse,
};
use features_payments_stripe_model::stripe_payment_intent::StripePaymentIntentForCreateRequest;
use features_payments_stripe_model::stripe_refund::StripeRefundForCreateRequest;
use features_payments_stripe_model::stripe_webhook_event::StripeWebhookEventForCreateRequest;

use crate::{StripePaymentIntentService, StripeRefundService, StripeWebhookEventService};

pub struct PaymentFlowService;

impl PaymentFlowService {
    pub async fn initiate_payment(
        client: &stripe::Client,
        baggage: &str,
        req: InitiatePaymentRequest,
    ) -> Result<InitiatePaymentResponse, AppError> {
        debug!(
            "Initiating payment for user_id: {}, amount: {}, currency: {}",
            req.user_id, req.amount, req.currency
        );
        // 1. Create core payment via remote
        let payment_id = PaymentRemoteService::create_payment(
            baggage,
            PaymentForCreateRequest {
                user_id: req.user_id,
                amount: req.amount,
                currency: req.currency.clone(),
                provider_name: "stripe".to_string(),
                idempotency_key: req.idempotency_key.clone(),
                transaction_id: req.idempotency_key.clone(),
                gateway_transaction_id: "N/A".to_string(),
                metadata: req.metadata.clone(),
            },
        )
        .await
        .map_err(remote_err)?;

        // 2. Call Stripe API — if this fails, mark core payment as failed
        let currency = req
            .currency
            .parse::<stripe::Currency>()
            .map_err(|_| AppError::Internal(format!("Unsupported currency: {}", req.currency)))?;

        let mut params = stripe::CreatePaymentIntent::new(req.amount, currency);
        params.payment_method_types = Some(vec!["card".to_string()]);
        if let Some(ref meta) = req.metadata {
            if let Ok(map) =
                serde_json::from_value::<std::collections::HashMap<String, String>>(meta.clone())
            {
                params.metadata = Some(map);
            }
        }

        let pi = match stripe::PaymentIntent::create(client, params).await {
            Ok(pi) => pi,
            Err(e) => {
                error!("Stripe CreatePaymentIntent failed: {e}");
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
                        provider: "stripe".to_string(),
                        raw_request: json!({"amount": req.amount, "currency": req.currency}),
                        raw_response: json!(null),
                        success: false,
                        error_message: e.to_string(),
                    },
                )
                .await;
                return Err(AppError::Internal(
                    "Failed to create Stripe PaymentIntent".to_string(),
                ));
            }
        };

        let pi_id = pi.id.to_string();
        let client_secret = pi.client_secret.clone().unwrap_or_default();
        let status = format!("{:?}", pi.status);

        // 3. Update core payment with gateway_transaction_id
        if let Err(e) = PaymentRemoteService::update_payment(
            baggage,
            payment_id,
            PaymentForUpdateRequest {
                status: Some("processing".to_string()),
                gateway_transaction_id: Some(pi_id.clone()),
            },
        )
        .await
        {
            error!("Failed to update core payment {payment_id} with PI {pi_id}: {e}");
        }

        // 4. Persist stripe payment intent record
        if let Err(e) =
            StripePaymentIntentService::create_payment_intent(StripePaymentIntentForCreateRequest {
                payment_id,
                stripe_payment_intent_id: pi_id.clone(),
                amount: req.amount,
                currency: req.currency.clone(),
                status: status.clone(),
                client_secret: client_secret.clone(),
                metadata: req.metadata,
            })
            .await
        {
            error!("Failed to persist stripe payment intent record for PI {pi_id}: {e}");
        }

        // 5. Log attempt via remote
        let _ = PaymentAttemptRemoteService::create_payment_attempt(
            baggage,
            PaymentAttemptForCreateRequest {
                payment_id,
                provider: "stripe".to_string(),
                raw_request: json!({"amount": req.amount, "currency": req.currency}),
                raw_response: json!({"payment_intent_id": pi_id, "status": status}),
                success: true,
                error_message: String::new(),
            },
        )
        .await;

        Ok(InitiatePaymentResponse {
            payment_id,
            stripe_payment_intent_id: pi_id,
            client_secret,
        })
    }

    pub async fn process_webhook(
        payload: &str,
        signature: &str,
        webhook_secret: &str,
        baggage: &str,
    ) -> Result<(), AppError> {
        let event =
            stripe::Webhook::construct_event(payload, signature, webhook_secret).map_err(|e| {
                AppError::Internal(format!("Webhook signature verification failed: {e}"))
            })?;

        let event_id = event.id.to_string();
        let event_type = format!("{:?}", event.type_);

        let payload_json: serde_json::Value = serde_json::from_str(payload).unwrap_or_default();
        let event_data_json = payload_json.get("data").cloned().unwrap_or_default();

        let _ =
            StripeWebhookEventService::create_webhook_event(StripeWebhookEventForCreateRequest {
                stripe_event_id: event_id,
                event_type: event_type.clone(),
                event_data: event_data_json.clone(),
                processed: Some(false),
                processing_error: None,
            })
            .await;

        let new_status = match event.type_ {
            stripe::EventType::PaymentIntentSucceeded => Some("succeeded"),
            stripe::EventType::PaymentIntentPaymentFailed => Some("failed"),
            stripe::EventType::PaymentIntentCanceled => Some("canceled"),
            stripe::EventType::PaymentIntentProcessing => Some("processing"),
            _ => {
                debug!("Unhandled event type: {event_type}");
                None
            }
        };

        let pi_id = event_data_json
            .get("object")
            .and_then(|obj| obj.get("id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let (Some(pi_id), Some(status)) = (pi_id, new_status) {
            Self::update_payment_status_by_pi(&pi_id, status, baggage).await?;
        }

        Ok(())
    }

    pub async fn refund_payment(
        client: &stripe::Client,
        baggage: &str,
        req: RefundPaymentRequest,
    ) -> Result<RefundPaymentResponse, AppError> {
        let payment = PaymentRemoteService::get_payment_by_id(baggage, req.payment_id)
            .await
            .map_err(remote_err)?;
        let stripe_pi_id = payment
            .gateway_transaction_id
            .ok_or_else(|| AppError::Internal("Payment has no Stripe PaymentIntent".to_string()))?;

        let pi_id = stripe::PaymentIntentId::from_str(&stripe_pi_id)
            .map_err(|_| AppError::Internal("Invalid PaymentIntent ID".to_string()))?;

        let mut params = stripe::CreateRefund::new();
        params.payment_intent = Some(pi_id);
        params.amount = req.amount;

        let refund = stripe::Refund::create(client, params).await.map_err(|e| {
            error!("Stripe CreateRefund failed: {e}");
            AppError::Internal("Failed to create Stripe refund".to_string())
        })?;

        let stripe_refund_id = refund.id.to_string();
        let refund_status = refund.status.unwrap_or_default();

        let refund_record_id = StripeRefundService::create_refund(StripeRefundForCreateRequest {
            payment_id: req.payment_id,
            stripe_refund_id: stripe_refund_id.clone(),
            stripe_payment_intent_id: stripe_pi_id,
            amount: refund.amount,
            currency: refund.currency.to_string(),
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
            stripe_refund_id,
            status: refund_status,
        })
    }

    async fn update_payment_status_by_pi(
        stripe_pi_id: &str,
        new_status: &str,
        baggage: &str,
    ) -> Result<(), AppError> {
        use features_payments_stripe_entities::stripe_payment_intent::Column;
        use sea_orm::Iden;
        use shared_shared_data_core::{
            filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
            order::Order,
            paging::Pagination,
        };

        let param: FilterParam<String> = FilterParam {
            name: Column::StripePaymentIntentId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(stripe_pi_id.to_string()),
            raw_value: stripe_pi_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(param)].into();
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let result =
            StripePaymentIntentService::get_payment_intents(&filters, &pagination, &order).await?;

        if let Some(pi_record) = result.result.first() {
            if let Some(pi_record_id) = pi_record.id {
                use features_payments_stripe_model::stripe_payment_intent::StripePaymentIntentForUpdateRequest;
                StripePaymentIntentService::update_payment_intent(
                    pi_record_id,
                    StripePaymentIntentForUpdateRequest {
                        status: Some(new_status.to_string()),
                        metadata: None,
                    },
                )
                .await?;

                if let Some(payment_id) = pi_record.payment_id {
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
                }
            }
        }

        Ok(())
    }
}
