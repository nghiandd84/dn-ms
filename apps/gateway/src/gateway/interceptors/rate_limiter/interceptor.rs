use std::time::Duration;

use async_trait::async_trait;
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

pub struct RateLimiterInterceptor {
    rate_limit: RateLimit,
    filter: Option<String>,
}

impl RateLimiterInterceptor {
    pub fn build(rate_limit: RateLimit, filter: Option<String>) -> Self {
        Self { rate_limit, filter }
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

    async fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        debug!(
            "Init RateLimiterInterceptor with filter {}",
            self.filter.as_ref().unwrap()
        );
        // TODO Implement rate limiter
        Ok(false)
    }
}
