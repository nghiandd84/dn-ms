use std::{collections::HashMap, sync::Arc};

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::interceptor::{Interceptor, InterceptorName},
};

pub trait InterceptorBuilder: Sync + Send {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>>;
}

pub struct InterceptorBuilderRegistry {
    pub registry: HashMap<InterceptorName, Arc<dyn InterceptorBuilder>>,
}

impl InterceptorBuilderRegistry {
    pub fn build() -> Self {
        let mut registry: HashMap<InterceptorName, Arc<dyn InterceptorBuilder>> = HashMap::new();

        Self { registry }
    }
}
