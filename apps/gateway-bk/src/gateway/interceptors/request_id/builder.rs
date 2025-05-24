use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
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
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let interceptor = RequestIdInterceptor::build(_interceptor_config.filter);
        Ok(Arc::new(interceptor))
    }
}
