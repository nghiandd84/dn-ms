mod config;
mod error;
mod gateway;
mod poller;

use dotenv::dotenv;
use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use pingora::{
    prelude::background_service,
    server::{configuration::ServerConf, Server},
};
use std::{sync::Arc, time::Duration};
use tracing::debug;


use shared_shared_observability::init_log_trace_metric;

use config::{app_config::load_app_config, dn_config::DnConfig, proxy::http::Proxy};
use gateway::{
    build_http,
    state::{build_gateway_state, GatewayStateStore},
};

use crate::poller::ApiPoller;

#[async_std::main]
async fn main() {
    dotenv().ok();
    let service_key = "GATEWAY".to_string();
    let (_log_provider, _trace_provider) =
        init_log_trace_metric(service_key).expect("Failed to initialize logging and tracing");
    global::set_text_map_propagator(TraceContextPropagator::new());

    let app_config = load_app_config();

    let dn_config = DnConfig::from_args(&app_config);
    let opt = dn_config.to_pingore_opt(&app_config);
    let config: ServerConf = dn_config.clone().into();

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
    // Create your background API poller
    let poller = ApiPoller {
        interval_duration: Duration::from_secs(60), // Call every 60 seconds
    };
    // Wrap it in Pingora's background service helper
    let background_task = background_service("API Poller", poller);
    server.add_service(background_task);

    debug!("Starting Gateway server...");
    server.run_forever();
}
