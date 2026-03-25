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
        crate::routes::wallet::update_wallet,
        crate::routes::wallet::delete_wallet,
        crate::routes::transaction::create_transaction,
        crate::routes::transaction::get_transaction,
        crate::routes::transaction::get_wallet_transactions,
        crate::routes::transaction::filter_transactions,
        crate::routes::transaction::update_transaction,
        crate::routes::transaction::delete_transaction,
    ),
    tags(
        (name = "wallet", description = "Wallet management endpoints"),
        (name = "transaction", description = "Transaction management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
