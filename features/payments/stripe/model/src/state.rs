use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PaymentsStripeAppState {}

impl Default for PaymentsStripeAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PaymentsStripeCacheState {
    Default,
}

impl Default for PaymentsStripeCacheState {
    fn default() -> Self {
        Self::Default
    }
}
