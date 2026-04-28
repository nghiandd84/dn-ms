use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// WALLET Permission
const WALLET_RESOURCE: &str = "WALLET_WALLET";

define_resource_perms! {
    CanCreateWallet => (CREATE, WALLET_RESOURCE),
    CanReadWallet => (READ, WALLET_RESOURCE),
    CanUpdateWallet => (UPDATE, WALLET_RESOURCE),
    CanDeleteWallet => (DELETE, WALLET_RESOURCE)
}

// TRANSACTION Permission
const TRANSACTION_RESOURCE: &str = "WALLET_TRANSACTION";

define_resource_perms! {
    CanCreateTransaction => (CREATE, TRANSACTION_RESOURCE),
    CanReadTransaction => (READ, TRANSACTION_RESOURCE)
}

// P2P_TRANSFER Permission
const P2P_TRANSFER_RESOURCE: &str = "WALLET_P2P_TRANSFER";

define_resource_perms! {
    CanCreateP2pTransfer => (CREATE, P2P_TRANSFER_RESOURCE),
    CanReadP2pTransfer => (READ, P2P_TRANSFER_RESOURCE)
}

// TOP_UP Permission
const TOP_UP_RESOURCE: &str = "WALLET_TOP_UP";

define_resource_perms! {
    CanCreateTopUp => (CREATE, TOP_UP_RESOURCE),
    CanReadTopUp => (READ, TOP_UP_RESOURCE)
}

// WITHDRAWAL Permission
const WITHDRAWAL_RESOURCE: &str = "WALLET_WITHDRAWAL";

define_resource_perms! {
    CanCreateWithdrawal => (CREATE, WITHDRAWAL_RESOURCE),
    CanReadWithdrawal => (READ, WITHDRAWAL_RESOURCE),
    CanUpdateWithdrawal => (UPDATE, WITHDRAWAL_RESOURCE)
}

// IDEMPOTENCY Permission
const IDEMPOTENCY_RESOURCE: &str = "WALLET_IDEMPOTENCY";

define_resource_perms! {
    CanCreateIdempotency => (CREATE, IDEMPOTENCY_RESOURCE),
    CanReadIdempotency => (READ, IDEMPOTENCY_RESOURCE),
    CanUpdateIdempotency => (UPDATE, IDEMPOTENCY_RESOURCE)
}
