use std::sync::Arc;

use pingora::{server::configuration::ServerConf, services::listening::Service};
use pingora_proxy::{http_proxy_service_with_name, HttpProxy};
use state::GatewayStateStore;

pub mod state;
pub mod interceptor;
pub mod interceptor_builder;
pub mod interceptors;

use super::Proxy;

pub type HttpGateway = Service<HttpProxy<Proxy>>;

pub async fn build_http(
    gateway_state_store: Arc<GatewayStateStore>,
    server_conf: Arc<ServerConf>,
) -> HttpGateway {
    let proxy = Proxy::build(gateway_state_store.clone()).await;
    let gateway_state = &gateway_state_store.get_state();
    let gateway_config = gateway_state.gateway_config();

    let mut http_proxy_service =
        http_proxy_service_with_name(&server_conf, proxy, gateway_config.name.as_str());
    let binding_address = &gateway_config.bind_addresses;

    for inet_address in binding_address {
        let addr = inet_address.get_formatted_address();
        http_proxy_service.add_tcp(&addr);
    }

    http_proxy_service
}
