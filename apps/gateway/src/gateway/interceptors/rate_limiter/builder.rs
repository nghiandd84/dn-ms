use std::{sync::Arc, time::Duration};

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        interceptor::Interceptor,
        interceptor_builder::InterceptorBuilder,
        interceptors::rate_limiter::interceptor::{RateLimit, RateLimiterInterceptor},
    },
    qe::query::extract_key_i64_or_err,
};

pub struct RateLimiterInterceptorBuilder {}

impl Default for RateLimiterInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for RateLimiterInterceptorBuilder {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let config = &_interceptor_config.config.expect(
            format!(
                "{:?} interceptor config not found.",
                _interceptor_config.name
            )
            .as_str(),
        );

        let capacity = extract_key_i64_or_err(config, "capacity")?;
        let refill_rate = extract_key_i64_or_err(config, "refill_rate")?;
        let refill_interval = extract_key_i64_or_err(config, "refill_interval")?;

        let rate_limit = RateLimit {
            capacity: capacity as u32,
            refill_rate: refill_rate as u32,
            refill_interval: Duration::from_millis(refill_interval as u64),
        };

        let interceptor = RateLimiterInterceptor::build(rate_limit);
        Ok(Arc::new(interceptor))
    }
}
