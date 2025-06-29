use std::{collections::HashMap, sync::Arc};

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::{Interceptor, InterceptorType},
        interceptors::{
            rate_limiter::RateLimiterInterceptorBuilder, request_id::RequestIdInterceptorBuilder,
        },
    },
};

pub mod utils;

pub trait InterceptorBuilder: Sync + Send {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>>;
}

pub struct InterceptorBuilderRegistry {
    pub registry: HashMap<InterceptorType, Arc<dyn InterceptorBuilder>>,
}

impl InterceptorBuilderRegistry {
    pub fn build() -> Self {
        let mut registry: HashMap<InterceptorType, Arc<dyn InterceptorBuilder>> = HashMap::new();

        registry.insert(
            InterceptorType::RequestId,
            Arc::new(RequestIdInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorType::RateLimiter,
            Arc::new(RateLimiterInterceptorBuilder::default()),
        );

        Self { registry }
    }
}
