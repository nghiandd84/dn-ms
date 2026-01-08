use std::{sync::Arc, time::Duration};

use tracing::debug;
use tracing_subscriber::field::debug;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor,
        interceptor_builder::InterceptorBuilder,
        interceptors::rate_limiter::interceptor::{RateLimit, RateLimiterInterceptor},
    },
};

pub struct RateLimiterInterceptorBuilder {}

impl Default for RateLimiterInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for RateLimiterInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        let config = interceptor_config.config.unwrap_or_default();
        let rate_limit = RateLimit {
            capacity: config
                .get("capacity")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(5),
            refill_rate: config
                .get("refill_rate")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(1),
            refill_interval: Duration::from_secs(
                config
                    .get("refill_interval")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(10),
            ),
        };
        debug!("Rate limit config: {:?}", debug(&rate_limit));
        let filter_name = interceptor_config.filter;
        let interceptor = RateLimiterInterceptor::build(rate_limit, filter_name);

        Ok(Arc::new(interceptor))
    }
}
