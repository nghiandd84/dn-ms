mod admin;
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
use tracing::{debug, info, warn};

use shared_shared_observability::init_log_trace_metric;

use config::{app_config::load_app_config, dn_config::DnConfig, proxy::http::Proxy};
use gateway::{
    build_http,
    state::{build_gateway_state, GatewayStateStore},
};

use crate::{admin::{admin_router, AdminState}, poller::ApiPoller};

#[async_std::main]
async fn main() {
    dotenv().ok();
    let service_key = "GATEWAY".to_string();
    let (_log_provider, _trace_provider, _metrics_provider) =
        init_log_trace_metric(service_key).expect("Failed to initialize logging and tracing");
    global::set_text_map_propagator(TraceContextPropagator::new());

    let app_config = load_app_config();

    let dn_config = DnConfig::from_args(&app_config);
    let opt = dn_config.to_pingore_opt(&app_config);
    let config: ServerConf = dn_config.clone().into();

    let mut server = Server::new_with_opt_and_conf(opt, config);

    server.bootstrap();

    let dn_config_clone = dn_config.clone();
    let mut gateway_stores: Vec<Arc<GatewayStateStore>> = Vec::new();

    for gateway_config in &dn_config_clone.gateways {
        let clone_gateway_config = gateway_config.clone();
        let gateway_state: gateway::state::GatewayState = build_gateway_state(clone_gateway_config);
        let gateway_state_store = Arc::new(GatewayStateStore::new(gateway_state));
        let server_conf: ServerConf = dn_config_clone.clone().into();
        let service = build_http(gateway_state_store.clone(), Arc::new(server_conf)).await;
        server.add_service(service);
        gateway_stores.push(gateway_state_store);
    }

    // Start admin API server for hot-reload
    let admin_api_key = app_config.admin_api_key.clone();
    if admin_api_key.is_none() {
        warn!("GATEWAY_ADMIN_API_KEY not set — admin reload endpoint is open (no auth required)");
    }
    let admin_state = Arc::new(AdminState {
        dp: app_config.dp.clone(),
        gateway_stores,
        admin_api_key,
    });
    let admin_port = app_config.admin_port;
    let admin_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", admin_port))
        .await
        .expect("Failed to bind admin port");
    info!("Admin API listening on 0.0.0.0:{}", admin_port);

    let _admin_handle = tokio::spawn(async move {
        axum::serve(admin_listener, admin_router(admin_state))
            .await
            .expect("Admin server failed");
    });

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
