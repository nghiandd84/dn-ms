use serde::{Deserialize, Serialize};

// --- Fingerprint models ---

#[derive(Deserialize)]
pub struct RegisterFingerprintRequest {
    pub fingerprint: String,
    pub user_agent: Option<String>,
    pub language: Option<String>,
    pub platform: Option<String>,
    pub screen_resolution: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FingerprintEntry {
    pub fingerprint: String,
    pub user_agent: Option<String>,
    pub language: Option<String>,
    pub platform: Option<String>,
    pub screen_resolution: Option<String>,
    pub timezone: Option<String>,
    pub ip_address: String,
    pub created_at: String,
    pub last_seen_at: String,
    pub request_count: u64,
    pub is_blocked: bool,
}

#[derive(Serialize)]
pub struct FingerprintListResponse {
    pub fingerprints: Vec<FingerprintEntry>,
    pub total: usize,
}

// --- Block models ---

#[derive(Deserialize)]
pub struct BlockRequest {
    pub fingerprint: String,
    pub reason: String,
    #[serde(default = "default_duration")]
    pub duration_seconds: u64,
    pub blocked_by: Option<String>,
}

fn default_duration() -> u64 {
    3600
}

#[derive(Serialize, Deserialize)]
pub struct BlockEntry {
    pub fingerprint: String,
    pub reason: String,
    pub duration_seconds: u64,
    pub blocked_by: Option<String>,
    pub blocked_at: String,
}

#[derive(Serialize)]
pub struct BlockListResponse {
    pub blocks: Vec<BlockEntry>,
    pub total: usize,
}
