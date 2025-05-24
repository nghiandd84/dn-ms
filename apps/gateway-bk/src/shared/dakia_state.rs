use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{
    config::DakiaConfig,
    error::{DakiaError, DakiaResult},
    gateway::state::{GatewayState, GatewayStateStore},
};

#[derive(Clone)]
pub struct DakiaState {
    dakia_config: DakiaConfig,
    gateway_state_stores: Vec<Arc<GatewayStateStore>>,
}

impl Default for DakiaState {
    fn default() -> Self {
        Self {
            dakia_config: Default::default(),
            gateway_state_stores: Default::default(),
        }
    }
}

pub static DAKIA_STATE: Lazy<Mutex<DakiaState>> = Lazy::new(|| Mutex::new(DakiaState::default()));

pub struct DakiaStateStore {}

impl DakiaStateStore {
    pub fn get_dakia_config(&self) -> DakiaResult<DakiaConfig> {
        match DAKIA_STATE.lock() {
            Ok(dakia_state) => Ok(dakia_state.dakia_config.clone()),
            Err(err) => Err(DakiaError::i_explain(format!(
                "Failed to acquire lock while retrieving Dakia config: {err}"
            ))),
        }
    }

    pub fn store_dakia_config(&self, dakia_config: DakiaConfig) -> DakiaResult<()> {
        match DAKIA_STATE.lock() {
            Ok(mut dakia_state) => {
                dakia_state.dakia_config = dakia_config;
                Ok(())
            }
            Err(err) => Err(DakiaError::i_explain(format!(
                "Failed to acquire lock while updating Dakia config: {err}"
            ))),
        }
    }

    pub fn get_gateway_stores(&self) -> DakiaResult<Vec<Arc<GatewayStateStore>>> {
        match DAKIA_STATE.lock() {
            Ok(dakia_state) => Ok(dakia_state.gateway_state_stores.clone()),
            Err(err) => Err(DakiaError::i_explain(format!(
                "Failed to acquire lock while retrieving gateway state stores: {err}"
            ))),
        }
    }

    pub fn store_gateway_state_stores(
        &self,
        gateway_stores: Vec<Arc<GatewayStateStore>>,
    ) -> DakiaResult<()> {
        match DAKIA_STATE.lock() {
            Ok(mut dakia_state) => {
                dakia_state.gateway_state_stores = gateway_stores;
                Ok(())
            }
            Err(err) => Err(DakiaError::i_explain(format!(
                "Failed to acquire lock while updating gateway state stores: {err}"
            ))),
        }
    }

    pub fn update_gateway_state(&self, gateway_state: GatewayState) -> DakiaResult<bool> {
        match DAKIA_STATE.lock() {
            Ok(dakia_state) => {
                let cloned_gateway_state = gateway_state.clone();
                let gateway_name = &cloned_gateway_state.gateway_config().name;

                for cur_gateway_state_store in &dakia_state.gateway_state_stores {
                    let cur_gateway_state = &cur_gateway_state_store.get_state();
                    let cur_gateway_name = &cur_gateway_state.gateway_config().name;
                    if cur_gateway_name == gateway_name {
                        cur_gateway_state_store.update_state(gateway_state.clone());
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            Err(err) => Err(DakiaError::i_explain(format!(
                "Failed to acquire lock while updating gateway state store: {err}"
            ))),
        }
    }
}

pub static DAKIA_STATE_STORE: Lazy<DakiaStateStore> = Lazy::new(|| DakiaStateStore {});
