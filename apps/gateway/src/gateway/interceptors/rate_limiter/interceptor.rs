use std::time::Duration;

use async_trait::async_trait;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
};

pub struct RateLimit {
    pub capacity: u32,
    pub refill_rate: u32,
    pub refill_interval: Duration,
}

pub struct RateLimiterInterceptor {
    rate_limit: RateLimit,
}

impl RateLimiterInterceptor {
    pub fn build(rate_limit: RateLimit) -> Self {
        Self { rate_limit }
    }
}

#[async_trait]
impl Interceptor for RateLimiterInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::RateLimiter
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    async fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        // TODO Implement rate limiter
        Ok(false)
    }
}
