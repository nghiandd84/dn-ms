use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Wallet API",
        version = "0.1.0",
        description = "Complete Wallet Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::wallet::create_wallet,
        crate::routes::wallet::get_wallet,
        crate::routes::wallet::get_wallet_by_user,
        crate::routes::wallet::filter_wallets,
        crate::routes::wallet::get_wallet_transactions,
        crate::routes::wallet::get_wallet_top_up_transactions,
        crate::routes::wallet::update_wallet,
        crate::routes::wallet::delete_wallet,
        crate::routes::top_up_transaction::create_top_up_transaction,
        crate::routes::top_up_transaction::get_top_up_transaction,
        crate::routes::top_up_transaction::filter_top_up_transactions,
        crate::routes::top_up_transaction::update_top_up_transaction,
        crate::routes::top_up_transaction::delete_top_up_transaction,
        crate::routes::transaction::create_transaction,
        crate::routes::transaction::get_transaction,
        crate::routes::transaction::filter_transactions,
        crate::routes::transaction::update_transaction,
        crate::routes::transaction::delete_transaction,
        crate::routes::p2p_transfer::create_p2p_transfer,
        crate::routes::p2p_transfer::get_p2p_transfer,
        crate::routes::p2p_transfer::filter_p2p_transfers,
        crate::routes::p2p_transfer::update_p2p_transfer,
        crate::routes::p2p_transfer::delete_p2p_transfer,
        crate::routes::withdrawal::create_withdrawal,
        crate::routes::withdrawal::get_withdrawal,
        crate::routes::withdrawal::filter_withdrawals,
        crate::routes::withdrawal::update_withdrawal,
        crate::routes::withdrawal::delete_withdrawal,
        crate::routes::idempotency::create_idempotency_key,
        crate::routes::idempotency::get_idempotency_keys,
        crate::routes::idempotency::get_idempotency_key_by_id,
        crate::routes::idempotency::update_idempotency_key,
        crate::routes::idempotency::delete_idempotency_key,
        crate::routes::idempotency::get_idempotency_key_by_key,
    ),
    tags(
        (name = "wallet", description = "Wallet management endpoints"),
        (name = "transaction", description = "Transaction management endpoints"),
        (name = "top_up", description = "Top-up transaction endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
