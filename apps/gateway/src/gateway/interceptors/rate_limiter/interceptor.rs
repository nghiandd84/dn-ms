use std::time::{Duration, Instant};

use async_trait::async_trait;
use dashmap::DashMap;
use pingora_http::ResponseHeader;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

#[derive(Debug)]
pub struct RateLimit {
    pub capacity: u32,
    pub refill_rate: u32,
    pub refill_interval: Duration,
}

struct Bucket {
    tokens: f64,
    last_refill: Instant,
}

pub struct RateLimiterInterceptor {
    rate_limit: RateLimit,
    filter: Option<String>,
    buckets: DashMap<String, Bucket>,
}

impl RateLimiterInterceptor {
    pub fn build(rate_limit: RateLimit, filter: Option<String>) -> Self {
        Self {
            rate_limit,
            filter,
            buckets: DashMap::new(),
        }
    }

    fn try_acquire(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut entry = self.buckets.entry(key.to_string()).or_insert_with(|| Bucket {
            tokens: self.rate_limit.capacity as f64,
            last_refill: now,
        });

        let bucket = entry.value_mut();
        let elapsed = now.duration_since(bucket.last_refill);
        let refill = elapsed.as_secs_f64() / self.rate_limit.refill_interval.as_secs_f64()
            * self.rate_limit.refill_rate as f64;

        if refill > 0.0 {
            bucket.tokens = (bucket.tokens + refill).min(self.rate_limit.capacity as f64);
            bucket.last_refill = now;
        }

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

#[async_trait]
impl Interceptor for RateLimiterInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::RateLimiter
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        let key = session
            .get_psession()
            .client_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        debug!(
            "RateLimiterInterceptor: client={}, filter={:?}",
            key, self.filter
        );

        if self.try_acquire(&key) {
            return Ok(false);
        }

        debug!("RateLimiterInterceptor: rate limit exceeded for {}", key);

        let psession = session.get_psession();
        let mut resp =
            ResponseHeader::build(http::StatusCode::TOO_MANY_REQUESTS, None).unwrap();
        let _ = resp.insert_header("Retry-After", self.rate_limit.refill_interval.as_secs().to_string());
        let _ = resp.insert_header("Content-Type", "text/plain");
        psession.set_keepalive(None);
        let _ = psession
            .write_response_header(Box::new(resp), false)
            .await;
        let _ = psession
            .write_response_body(Some(bytes::Bytes::from("Too Many Requests")), true)
            .await;

        Ok(true)
    }
}
