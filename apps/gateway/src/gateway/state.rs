use std::sync::Arc;

use arc_swap::ArcSwap;
use tracing::debug;

use crate::{
    config::{proxy::http::load_balancer::UpStreamLoadBalaner, source_config::GatewayConfig},
    gateway::{
        interceptor::{Interceptor, Phase},
        interceptor_builder::{utils::build_interceptors, InterceptorBuilderRegistry},
    },
};

#[derive(Clone)]
pub struct GatewayState {
    gateway_config: GatewayConfig,
    interceptors: Vec<Arc<dyn Interceptor>>,
    upstream_load_balancers: Arc<Vec<UpStreamLoadBalaner>>,
}

impl GatewayState {
    pub fn build(gateway_config: GatewayConfig) -> Self {
        let interceptor_builder_registry = InterceptorBuilderRegistry::build();
        let interceptors =
            build_interceptors(&gateway_config, &interceptor_builder_registry).unwrap_or_default();
        debug!("Loaded {} interceptors", interceptors.len());

        let upstream_load_balancers =
            UpStreamLoadBalaner::from_upstream_config_sync(gateway_config.upstreams.clone());

        Self {
            gateway_config,
            interceptors,
            upstream_load_balancers: Arc::new(upstream_load_balancers),
        }
    }

    pub fn gateway_config(&self) -> &GatewayConfig {
        &self.gateway_config
    }

    pub fn interceptors(&self) -> &Vec<Arc<dyn Interceptor>> {
        &self.interceptors
    }

    pub fn upstream_load_balancers(&self) -> Arc<Vec<UpStreamLoadBalaner>> {
        self.upstream_load_balancers.clone()
    }

    pub fn get_interceptors(&self, phase: Phase, filter_name: String) -> Vec<Arc<dyn Interceptor>> {
        self.interceptors
            .iter()
            .filter(|interceptor| {
                let is_match_phase = interceptor.phase_mask() & phase.mask() != 0;
                let default_filter = String::from("");
                let interceptor_filter = interceptor.filter().as_ref().unwrap_or(&default_filter);
                let is_match_filter = *interceptor_filter == filter_name;
                is_match_phase && is_match_filter
            })
            .cloned()
            .collect()
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

    pub async fn update_state(&self, new_state: GatewayState) {
        self.state.swap(Arc::new(new_state));
    }

    pub fn get_state(&self) -> Arc<GatewayState> {
        self.state.load_full()
    }
}

pub fn build_gateway_state(gateway_config: GatewayConfig) -> GatewayState {
    let gateway_state = GatewayState::build(gateway_config);
    debug!("Gateway state loaded with interceptors and load balancers");
    gateway_state
}
