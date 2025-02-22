use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{interceptor::Interceptor, interceptor_builder::InterceptorBuilder},
};

use super::{interceptor::RequestRewriteInterceptor, rewrite_parts::RewriteParts};

pub struct RequestRewriteInterceptorBuilder {}

impl Default for RequestRewriteInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for RequestRewriteInterceptorBuilder {
    fn build(
        &self,
        _interceptor_config: InterceptorConfig,
    ) -> DakiaResult<std::sync::Arc<dyn Interceptor>> {
        let rewrite_parts = RewriteParts::build(&_interceptor_config)?;
        let interceptor =
            RequestRewriteInterceptor::build(_interceptor_config.filter, rewrite_parts);
        Ok(Arc::new(interceptor))
    }
}
