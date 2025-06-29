use std::{sync::Arc, time::Duration};

use tracing::debug;

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
        // TODO read config and create rate_limit
        debug!("RateLimiterInterceptorBuilder");
        let rate_limit = RateLimit {
            capacity: 1,
            refill_rate: 1,
            refill_interval: Duration::from_secs(10),
        };
        let filter_name = interceptor_config.filter;
        let interceptor = RateLimiterInterceptor::build(rate_limit, filter_name);

        Ok(Arc::new(interceptor))
    }
}
