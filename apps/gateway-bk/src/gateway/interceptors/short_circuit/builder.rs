use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        interceptor::Interceptor,
        interceptor_builder::InterceptorBuilder,
        interceptors::short_circuit::{
            interceptor::ShortCircuitInterceptor, response_parts::ResponseParts,
        },
    },
};

pub struct ShortCircuitInterceptorBuilder {}

impl Default for ShortCircuitInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for ShortCircuitInterceptorBuilder {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let response_parts = ResponseParts::build(&_interceptor_config)?;

        let interceptor =
            ShortCircuitInterceptor::build(_interceptor_config.filter, response_parts);

        Ok(Arc::new(interceptor))
    }
}
