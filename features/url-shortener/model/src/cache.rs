use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Minimal data cached in Redis for redirect lookups.
/// Contains only what's needed to perform a redirect + validation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedUrlData {
    pub url_id: Uuid,
    pub original_url: String,
    pub expires_at: Option<DateTime>,
    pub is_active: bool,
}
