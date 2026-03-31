use serde::{Deserialize, Serialize};

use shared_shared_extractor::IdempotencyCacheType;

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletAppState {}

impl Default for WalletAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WalletCacheState {
    Default,
    IdempotencyValue(bool),
}

impl Default for WalletCacheState {
    fn default() -> Self {
        Self::Default
    }
}

impl IdempotencyCacheType for WalletCacheState {
    fn default_idempotency_value() -> Self {
        WalletCacheState::IdempotencyValue(true)
    }
}
