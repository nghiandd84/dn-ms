use std::sync::Arc;

use tracing::debug;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::request_id::interceptor::RequestIdInterceptor,
    },
};

pub struct RequestIdInterceptorBuilder {}

impl Default for RequestIdInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for RequestIdInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        debug!("RequestIdInterceptorBuilder");
        let interceptor = RequestIdInterceptor::build(interceptor_config.filter);
        Ok(Arc::new(interceptor))
    }
}
