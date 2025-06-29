use std::sync::Arc;

use arc_swap::ArcSwap;
use tracing::debug;

use crate::config::source_config::GatewayConfig;

#[derive(Clone, Debug)]
pub struct GatewayState {
    gateway_config: GatewayConfig,
}

impl GatewayState {
    pub fn build(gateway_config: GatewayConfig) -> Self {
        Self {
            gateway_config: gateway_config,
        }
    }

    pub fn gateway_config(&self) -> &GatewayConfig {
        &self.gateway_config
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
}

pub fn build_gateway_state(gateway_config: GatewayConfig) -> GatewayState {
    let gateway_state = GatewayState::build(gateway_config);
    debug!("Gateway state {:?}", gateway_state);
    gateway_state
}
