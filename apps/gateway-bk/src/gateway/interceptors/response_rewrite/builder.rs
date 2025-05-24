use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{interceptor::Interceptor, interceptor_builder::InterceptorBuilder},
};

use super::{interceptor::ResponseRewriteInterceptor, rewrite_parts::RewriteParts};

pub struct ResponseRewriteInterceptorBuilder {}

impl Default for ResponseRewriteInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for ResponseRewriteInterceptorBuilder {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let rewrite_parts = RewriteParts::build(&_interceptor_config)?;
        let interceptor =
            ResponseRewriteInterceptor::build(_interceptor_config.filter, rewrite_parts);
        Ok(Arc::new(interceptor))
    }
}
