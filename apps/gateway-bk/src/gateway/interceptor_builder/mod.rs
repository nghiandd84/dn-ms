pub mod utils;
use std::{collections::HashMap, sync::Arc};

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::interceptor::{Interceptor, InterceptorName},
};

use super::interceptors::{
    basic_auth::BasicAuthInterceptorBuilder, controller::ControllerInterceptorBuilder,
    rate_limiter::RateLimiterInterceptorBuilder, request_id::RequestIdInterceptorBuilder,
    request_rewrite::RequestRewriteInterceptorBuilder,
    response_rewrite::ResponseRewriteInterceptorBuilder, server_version,
    short_circuit::ShortCircuitInterceptorBuilder, use_file,
};

pub trait InterceptorBuilder: Sync + Send {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>>;
}

#[derive(Clone)]
pub struct InterceptorBuilderRegistry {
    /*
    Arc<dyn InterceptorBuilder> used instead of Box<dyn InterceptorBuilder> because of the error the trait `InterceptorBuilder` cannot be made into an object `InterceptorBuilder` cannot be made into an objec
    - https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
    - https://www.reddit.com/r/rust/comments/7q3bz8/trait_object_with_clone/
    - https://stackoverflow.com/questions/64725210/how-to-make-a-trait-and-a-struct-implementing-it-clonable
    */
    // Mutex does not support Clone so wrapped in Arc
    pub registry: HashMap<InterceptorName, Arc<dyn InterceptorBuilder>>,
}

impl InterceptorBuilderRegistry {
    pub fn build() -> Self {
        let mut registry: HashMap<InterceptorName, Arc<dyn InterceptorBuilder>> = HashMap::new();

        registry.insert(
            InterceptorName::ServerVersion,
            Arc::new(server_version::ServerVersionInterceptorBuilder::default()),
        );
        registry.insert(
            InterceptorName::UseFile,
            Arc::new(use_file::UseFileInterceptorBuilder::default()),
        );
        registry.insert(
            InterceptorName::BasicAuth,
            Arc::new(BasicAuthInterceptorBuilder::default()),
        );
        registry.insert(
            InterceptorName::Controller,
            Arc::new(ControllerInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorName::RateLimiter,
            Arc::new(RateLimiterInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorName::RequestRewrite,
            Arc::new(RequestRewriteInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorName::ResponseRewrite,
            Arc::new(ResponseRewriteInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorName::ShortCircuit,
            Arc::new(ShortCircuitInterceptorBuilder::default()),
        );

        registry.insert(
            InterceptorName::RequestId,
            Arc::new(RequestIdInterceptorBuilder::default()),
        );

        Self { registry }
    }
}
