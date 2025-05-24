use crate::{
    config::{source_config::GatewayConfig, ConfigVersion},
    error::{DakiaError, DakiaResult},
    shared::{mutable_registry::Registry, pattern_registry::PatternRegistryType},
};
use arc_swap::ArcSwap;
use std::sync::Arc;

use super::{
    filter::{build_filter_registry, Filter},
    interceptor::Interceptor,
    interceptor_builder::{utils::build_interceptors, InterceptorBuilderRegistry},
    lb, registry_builder,
};

#[derive(Clone)]
pub struct GatewayState {
    version: ConfigVersion,
    gateway_config: GatewayConfig,
    ds_host_pattern_registry: PatternRegistryType,
    lb_registry: lb::LbRegistryType,
    _interceptor_builder_registry: InterceptorBuilderRegistry,
    interceptors: Vec<Arc<dyn Interceptor>>,
    filter_registry: Registry<Filter>,
}

impl GatewayState {
    pub fn build(
        version: ConfigVersion,
        gateway_config: GatewayConfig,
        ds_host_pattern_registry: PatternRegistryType,
        lb_registry: lb::LbRegistryType,
        interceptor_builder_registry: InterceptorBuilderRegistry,
        interceptors: Vec<Arc<dyn Interceptor>>,
        filter_registry: Registry<Filter>,
    ) -> Self {
        Self {
            version,
            gateway_config,
            ds_host_pattern_registry,
            lb_registry,
            _interceptor_builder_registry: interceptor_builder_registry,
            interceptors,
            filter_registry,
        }
    }

    pub fn gateway_config(&self) -> &GatewayConfig {
        &self.gateway_config
    }

    pub fn pattern_registry(&self) -> &PatternRegistryType {
        &self.ds_host_pattern_registry
    }

    pub fn lb_registry(&self) -> &lb::LbRegistryType {
        &self.lb_registry
    }

    pub fn interceptors(&self) -> &Vec<Arc<dyn Interceptor>> {
        &self.interceptors
    }

    pub fn filter(&self, filter_name: &str) -> Option<&Filter> {
        self.filter_registry.get(filter_name)
    }

    pub fn filter_or_err(&self, filter_name: &str) -> DakiaResult<&Filter> {
        self.filter_registry
            .get(filter_name)
            .ok_or(DakiaError::i_explain(format!(
                "expected filter {filter_name} not found in filter registry"
            )))
    }

    pub fn version(&self) -> ConfigVersion {
        self.version
    }
}

pub struct GatewayStateStore {
    state: ArcSwap<GatewayState>,
}

impl GatewayStateStore {
    pub fn new(state: GatewayState) -> Self {
        Self {
            state: ArcSwap::new(Arc::new(state)),
        }
    }
}

impl GatewayStateStore {
    pub fn update_state(&self, new_state: GatewayState) -> () {
        self.state.swap(Arc::new(new_state));
    }

    pub fn get_state(&self) -> Arc<GatewayState> {
        self.state.load_full()
    }

    pub fn get_inner(&self) -> GatewayState {
        let arc_config = self.get_state().clone();
        (*arc_config).clone()
    }
}

pub async fn build_gateway_state(
    mut gateway_config: GatewayConfig,
    version: ConfigVersion,
) -> DakiaResult<GatewayState> {
    let ds_host_pattern_registry =
        registry_builder::build_ds_host_pattern_registry(&gateway_config).await?;
    let lb_registry = registry_builder::build_lb_registry(&gateway_config).await?;

    let interceptor_builder_registry = InterceptorBuilderRegistry::build();
    let filter_registry = build_filter_registry(&mut gateway_config)?;
    let interceptors = build_interceptors(&gateway_config, &interceptor_builder_registry)?;
    let gateway_state = GatewayState::build(
        version,
        gateway_config,
        ds_host_pattern_registry,
        lb_registry,
        interceptor_builder_registry,
        interceptors,
        filter_registry,
    );

    Ok(gateway_state)
}
