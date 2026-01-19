use std::sync::Arc;

use tracing::debug;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::cors::interceptor::CorsInterceptor,
    },
};

pub struct CorsInterceptorBuilder {}

impl Default for CorsInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for CorsInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        let config = interceptor_config.config.unwrap_or_default();
        debug!("Building CorsInterceptor with config: {:?}", config);

        let domains = config
            .get("allowed_domains")
            .and_then(|v| {
                let domains = v.parse::<String>().ok();
                debug!("Parsing allowed_domains for CorsInterceptor: {}", domains.as_deref().unwrap_or(""));
                domains
            })
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or(vec![]);
        let interceptor = CorsInterceptor::build(interceptor_config.filter, domains);
        Ok(Arc::new(interceptor))
    }
}
