use std::{
    cmp::min,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use dashmap::DashMap;
use http::StatusCode;

use crate::{
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

struct Bucket {
    last_refilled_time: Instant,
    tokens: u32,
}

impl Bucket {
    fn refill(&mut self, capacity: u32, rate_limit: &RateLimit) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refilled_time).as_millis();
        let rifill_cycles = elapsed / rate_limit.refill_interval.as_millis();
        let max_refill_tokens = rifill_cycles * rate_limit.refill_rate as u128;
        let total_tokens = max_refill_tokens + self.tokens as u128;
        let new_token_balance = min(capacity, total_tokens as u32);
        self.tokens = new_token_balance;
        self.last_refilled_time = Instant::now();
    }

    fn try_consume(&mut self) -> bool {
        if self.tokens > 0 {
            self.tokens = self.tokens - 1;
            true
        } else {
            false
        }
    }
}

pub struct RateLimit {
    pub capacity: u32,
    pub refill_rate: u32,
    pub refill_interval: Duration,
}

pub struct RateLimiterInterceptor {
    rate_limit: RateLimit,
    buckets: DashMap<String, Bucket>,
}

impl RateLimiterInterceptor {
    pub fn build(rate_limit: RateLimit) -> Self {
        Self {
            rate_limit,
            buckets: DashMap::new(),
        }
    }
}

// NOTE: only token bucket algorithm supported for now, it uses user ip to enfore rate limit to prevent DDoS attack
#[async_trait]
impl Interceptor for RateLimiterInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::RateLimiter
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    async fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        let socket_addr = _session
            .ds_socket_addr()
            .map_or("".to_string(), |socket_addr| socket_addr.to_string())
            .split(":") // keep only ip, ignore client port
            .next()
            .map(String::from)
            .map_or("".to_string(), |ip| ip);

        let mut bucket = match self.buckets.get_mut(&socket_addr) {
            Some(bucket) => bucket,
            None => {
                let new_bucket = Bucket {
                    last_refilled_time: Instant::now(),
                    tokens: self.rate_limit.refill_rate,
                };
                self.buckets.insert(socket_addr.clone(), new_bucket);

                // safe to unwrap, it'll be always available at this point
                self.buckets.get_mut(&socket_addr).unwrap()
            }
        };

        bucket.refill(self.rate_limit.capacity, &self.rate_limit);
        let consumed = bucket.try_consume();

        if !consumed {
            _session.set_res_status(StatusCode::TOO_MANY_REQUESTS);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
