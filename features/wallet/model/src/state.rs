use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WalletAppState {}

impl Default for WalletAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum WalletCacheState {}
