use std::sync::Arc;

use tracing::debug;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::token_auth::interceptor::TokenAuthInterceptor,
    },
};

pub struct TokenAuthInterceptorBuilder {}

impl Default for TokenAuthInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for TokenAuthInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        debug!("TokenAuthInterceptorBuilder");
        let interceptor = TokenAuthInterceptor::build(interceptor_config.filter);
        Ok(Arc::new(interceptor))
    }
}
