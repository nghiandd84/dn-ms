use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MerchantAppState {}

impl Default for MerchantAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MerchantCacheState {
    Default
}

impl Default for MerchantCacheState {
    fn default() -> Self {
        Self::Default
    }
}