use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PaymentsPaypalAppState {
    pub http_client: reqwest::Client,
    pub client_id: String,
    pub client_secret: String,
    pub api_base: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PaymentsPaypalCacheState {
    Default,
}

impl Default for PaymentsPaypalCacheState {
    fn default() -> Self {
        Self::Default
    }
}
