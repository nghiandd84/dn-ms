use std::sync::Arc;

use tracing::debug;

use crate::{
    config::source_config::{GatewayConfig, InterceptorConfig},
    error::{Error, GatewayResult},
    gateway::{interceptor::Interceptor, interceptor_builder::InterceptorBuilderRegistry},
};

fn build_interceptor(
    interceptor_config: &InterceptorConfig,
    interceptor_builder_registry: &InterceptorBuilderRegistry,
) -> GatewayResult<Arc<dyn Interceptor>> {
    let interceptor_name = &interceptor_config.interceptor_type;
    let builder = interceptor_builder_registry.registry.get(interceptor_name);

    let interceptor = match builder {
        Some(builder) => builder.build(interceptor_config.clone())?,
        None => {
            // let error_message = format!("Invalidate interceptor {}", interceptor_name.clone().as_str());
            // let error = Error::from_str(error_message.to_owned());
            let error = Error::from_str("Invalidate interceptor");
            return Err(Box::new(error));
        }
    };
    Ok(interceptor)
}

pub fn build_interceptors(
    gateway_config: &GatewayConfig,
    interceptor_builder_registry: &InterceptorBuilderRegistry,
) -> GatewayResult<Vec<Arc<dyn Interceptor>>> {
    let mut interceptors: Vec<Arc<dyn Interceptor>> = vec![];

    for interceptor_config in &gateway_config.interceptors {
        debug!(
            "Initializing interceptor: {:?} (enabled: {})",
            interceptor_config.name, interceptor_config.enabled
        );

        if !interceptor_config.enabled {
            continue;
        }

        let interceptor = build_interceptor(interceptor_config, interceptor_builder_registry)?;
        interceptors.push(interceptor);
    }

    Ok(interceptors)
}
