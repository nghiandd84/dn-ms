use once_cell::sync::Lazy;
use opentelemetry::{
    global,
    metrics::{Counter, Histogram},
    KeyValue,
};

/// Custom OpenTelemetry metrics for the URL shortener service.
pub struct UrlShortenerMetrics {
    /// Total number of redirects performed.
    pub redirects_total: Counter<u64>,
    /// Total number of URLs created.
    pub urls_created_total: Counter<u64>,
    /// Total number of expired/inactive redirect attempts.
    pub expired_redirects_total: Counter<u64>,
    /// Redirect resolution latency (seconds).
    pub redirect_latency: Histogram<f64>,
    /// Total Redis cache hits for redirect lookups.
    pub cache_hits_total: Counter<u64>,
    /// Total Redis cache misses for redirect lookups.
    pub cache_misses_total: Counter<u64>,
}

impl UrlShortenerMetrics {
    fn new() -> Self {
        let meter = global::meter("url_shortener");

        let redirects_total = meter
            .u64_counter("url_shortener.redirects_total")
            .with_description("Total number of redirect operations")
            .build();

        let urls_created_total = meter
            .u64_counter("url_shortener.urls_created_total")
            .with_description("Total number of shortened URLs created")
            .build();

        let expired_redirects_total = meter
            .u64_counter("url_shortener.expired_redirects_total")
            .with_description("Total number of expired or inactive redirect attempts")
            .build();

        let redirect_latency = meter
            .f64_histogram("url_shortener.redirect_latency")
            .with_description("Time to resolve a short code to redirect (seconds)")
            .with_unit("s")
            .build();

        let cache_hits_total = meter
            .u64_counter("url_shortener.cache_hits_total")
            .with_description("Total Redis cache hits for redirect lookups")
            .build();

        let cache_misses_total = meter
            .u64_counter("url_shortener.cache_misses_total")
            .with_description("Total Redis cache misses for redirect lookups")
            .build();

        Self {
            redirects_total,
            urls_created_total,
            expired_redirects_total,
            redirect_latency,
            cache_hits_total,
            cache_misses_total,
        }
    }
}

/// Global metrics instance, initialized lazily on first access.
pub static METRICS: Lazy<UrlShortenerMetrics> = Lazy::new(UrlShortenerMetrics::new);

/// Record a successful redirect.
pub fn record_redirect_success() {
    METRICS
        .redirects_total
        .add(1, &[KeyValue::new("status", "success")]);
}

/// Record an expired redirect attempt.
pub fn record_redirect_expired() {
    METRICS
        .expired_redirects_total
        .add(1, &[]);
    METRICS
        .redirects_total
        .add(1, &[KeyValue::new("status", "expired")]);
}

/// Record an inactive redirect attempt.
pub fn record_redirect_inactive() {
    METRICS
        .expired_redirects_total
        .add(1, &[]);
    METRICS
        .redirects_total
        .add(1, &[KeyValue::new("status", "inactive")]);
}

/// Record URL creation.
pub fn record_url_created(code_type: &str) {
    METRICS
        .urls_created_total
        .add(1, &[KeyValue::new("code_type", code_type.to_string())]);
}

/// Record a cache hit.
pub fn record_cache_hit() {
    METRICS.cache_hits_total.add(1, &[]);
}

/// Record a cache miss.
pub fn record_cache_miss() {
    METRICS.cache_misses_total.add(1, &[]);
}

/// Record redirect latency in seconds.
pub fn record_redirect_latency(duration_secs: f64) {
    METRICS.redirect_latency.record(duration_secs, &[]);
}
