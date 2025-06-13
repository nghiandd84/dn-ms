mod config;
mod error;
mod gateway;

use dotenv::dotenv;
use pingora::server::{configuration::ServerConf, Server};
use std::sync::Arc;

use config::{app_config::load_app_config, dn_config::DnConfig, proxy::http::Proxy};
use gateway::{
    build_http,
    state::{build_gateway_state, GatewayStateStore},
};

#[async_std::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let app_config = load_app_config();

    let dn_config = DnConfig::from_args(&app_config);
    let opt = dn_config.to_pingore_opt(&app_config);
    let config: ServerConf = dn_config.clone().into();

    // Runtime::new().unwrap().block_on(|| {
    let mut server = Server::new_with_opt_and_conf(opt, config);

    server.bootstrap();

    let dn_config_clone = dn_config.clone();
    for gateway_config in &dn_config_clone.gateways {
        let clone_gateway_config = gateway_config.clone();
        let gateway_state: gateway::state::GatewayState = build_gateway_state(clone_gateway_config);
        let gateway_state_store = Arc::new(GatewayStateStore::new(gateway_state));
        let server_conf: ServerConf = dn_config_clone.clone().into();
        let service = build_http(gateway_state_store, Arc::new(server_conf)).await;
        server.add_service(service);
    }

    server.run_forever();
    // });
}
