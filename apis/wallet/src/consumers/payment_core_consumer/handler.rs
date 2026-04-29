use std::collections::HashMap;
use sea_orm::Iden;
use tracing::{debug, error};

use shared_shared_app::state::AppState;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};

use features_payments_core_stream::{PaymentCoreEventMessage, PaymentSucceededMessage};
use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    transaction::TransactionForCreateRequest,
};
use features_wallet_service::{TransactionService, WalletService};

use features_wallet_entities::transaction::Column as TxColumn;

pub async fn handle_payment_core_message(
    message: PaymentCoreEventMessage,
    _state: AppState<WalletAppState, WalletCacheState>,
    _headers: Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match message {
        PaymentCoreEventMessage::Succeeded { message } => {
            handle_payment_succeeded(message).await
        }
    }
}

async fn handle_payment_succeeded(
    msg: PaymentSucceededMessage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let wallet_id = match msg.wallet_id {
        Some(id) => id,
        None => {
            debug!("No wallet_id in payment succeeded event, skipping. payment_id={}", msg.payment_id);
            return Ok(());
        }
    };

    let reference_id = format!("payment:{}", msg.payment_id);

    // Idempotency check: skip if transaction with this reference_id already exists
    let filters: FilterCondition = vec![FilterEnum::String(FilterParam {
        name: TxColumn::ReferenceId.to_string(),
        operator: FilterOperator::Equal,
        value: Some(reference_id.clone()),
        raw_value: reference_id.clone(),
    })]
    .into();
    let existing = TransactionService::get_transactions(
        &Pagination { page: Some(1), page_size: Some(1) },
        &Order::default(),
        &filters,
    )
    .await?;
    if !existing.result.is_empty() {
        debug!("Transaction already exists for reference_id={}, skipping", reference_id);
        return Ok(());
    }

    // Credit wallet
    let amount = msg.amount as f32;
    WalletService::credit_wallet(wallet_id, amount).await.map_err(|e| {
        error!("Failed to credit wallet {}: {:?}", wallet_id, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    // Create DEPOSIT transaction record
    let tx_request = TransactionForCreateRequest {
        wallet_id,
        transaction_type: "DEPOSIT".to_string(),
        amount,
        currency: msg.currency,
        reference_id: Some(reference_id.clone()),
        description: Some(format!("Payment credit from payment_id={}", msg.payment_id)),
    };
    TransactionService::create_transaction(tx_request).await.map_err(|e| {
        error!("Failed to create transaction for reference_id={}: {:?}", reference_id, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    debug!("Credited wallet {} with {} for payment {}", wallet_id, amount, msg.payment_id);
    Ok(())
}
