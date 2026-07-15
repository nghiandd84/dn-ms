use std::time::Duration;

use shared_shared_data_cache::cache::Cache;
use tracing::{debug, warn};

use features_url_shortener_model::cache::CachedUrlData;

const CACHE_KEY_PREFIX: &str = "url_shortener";
const CACHE_TTL_SECS: u64 = 300; // 5 minutes

/// Redis cache wrapper for URL shortener redirect lookups.
pub struct UrlShortenerCache;

impl UrlShortenerCache {
    fn get_cache() -> Option<Cache<String, CachedUrlData>> {
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        Cache::with_default_ttl(&redis_url, CACHE_KEY_PREFIX, Duration::from_secs(CACHE_TTL_SECS))
            .ok()
    }

    /// Get cached URL data by short code.
    pub fn get_url(code: &str) -> Option<CachedUrlData> {
        let cache = Self::get_cache()?;
        match cache.get(&code.to_string()) {
            Ok(data) => {
                if data.is_some() {
                    debug!("Cache HIT for short code: {}", code);
                }
                data
            }
            Err(e) => {
                warn!("Redis cache get error for code {}: {:?}", code, e);
                None
            }
        }
    }

    /// Cache URL data for a short code.
    pub fn set_url(code: &str, data: &CachedUrlData) {
        if let Some(cache) = Self::get_cache() {
            if let Err(e) = cache.insert(code.to_string(), data.clone(), None) {
                warn!("Redis cache set error for code {}: {:?}", code, e);
            } else {
                debug!("Cache SET for short code: {}", code);
            }
        }
    }

    /// Invalidate (remove) cached URL data for a short code.
    pub fn invalidate(code: &str) {
        if let Some(cache) = Self::get_cache() {
            if let Err(e) = cache.remove(&code.to_string()) {
                warn!("Redis cache invalidate error for code {}: {:?}", code, e);
            } else {
                debug!("Cache INVALIDATED for short code: {}", code);
            }
        }
    }
}
