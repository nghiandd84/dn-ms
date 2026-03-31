use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PaymentsCoreAppState {}

impl Default for PaymentsCoreAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PaymentsCoreCacheState {
    Default
}

impl Default for PaymentsCoreCacheState {
    fn default() -> Self {
        Self::Default
    }
}