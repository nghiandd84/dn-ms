use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::server_version::ServerVersionInterceptor,
    },
};

pub struct ServerVersionInterceptorBuilder {}

impl Default for ServerVersionInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for ServerVersionInterceptorBuilder {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let interceptor = ServerVersionInterceptor {};
        Ok(Arc::new(interceptor))
    }
}
