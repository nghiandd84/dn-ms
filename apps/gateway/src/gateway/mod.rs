pub mod filter;
pub mod interceptor;
pub mod interceptor_builder;
pub mod interceptors;
pub mod lb;
pub mod registry_builder;
pub mod state;

use super::Proxy;
use pingora::{server::configuration::ServerConf, services::listening::Service};
use pingora_proxy::{http_proxy_service_with_name, HttpProxy};
use state::GatewayStateStore;
use std::sync::Arc;

use crate::error::DakiaResult;

pub type HttpGateway = Service<HttpProxy<Proxy>>;

pub async fn build_http(
    gateway_state_store: Arc<GatewayStateStore>,
    server_conf: Arc<ServerConf>,
) -> DakiaResult<HttpGateway> {
    let proxy = Proxy::build(gateway_state_store.clone()).await?;
    let mut http_proxy_service =
        http_proxy_service_with_name(&server_conf, proxy, "Dakia HTTP Proxy");

    let gateway_state = &gateway_state_store.get_state();
    let bind_addresses = &gateway_state.gateway_config().bind_addresses;

    for inet_address in bind_addresses {
        let addr = inet_address.get_formatted_address();
        http_proxy_service.add_tcp(&addr);
    }

    Ok(http_proxy_service)
}
