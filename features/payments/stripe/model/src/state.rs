use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PaymentsStripeAppState {
    pub stripe_client: stripe::Client,
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
