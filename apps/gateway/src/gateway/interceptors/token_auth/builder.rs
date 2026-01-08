use std::sync::Arc;

use tracing::debug;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor,
        interceptor_builder::InterceptorBuilder,
        interceptors::token_auth::interceptor::{TokenAuthConfig, TokenAuthInterceptor},
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
        let config = interceptor_config.config.unwrap_or_default();
        let verify_endpoint = config
            .get("verify_endpoint")
            .expect("verify_endpoint is required");
        let token_auth_config = TokenAuthConfig {
            verify_endpoint: verify_endpoint.clone(),
        };
        debug!("Token auth config: {:?}", debug(&token_auth_config));
        let interceptor = TokenAuthInterceptor::build(token_auth_config, interceptor_config.filter);
        Ok(Arc::new(interceptor))
    }
}
