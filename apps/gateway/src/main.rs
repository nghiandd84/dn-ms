mod config;
mod gateway;
mod error;

use std::sync::Arc;

use gateway::{
    build_http,
    state::{build_gateway_state, GatewayStateStore},
};
use pingora::{
    lb::{selection, LoadBalancer},
    server::{configuration::ServerConf, Server},
};

use dotenv::dotenv;
use tracing::debug;

use config::{app_config::load_app_config, dn_config::DnConfig, proxy::http::Proxy};

fn main() {
    // Load .env file
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let app_config = load_app_config();
    /*
    let runtime = Builder::new_current_thread()
        .build()
        // if there is any error, just panic
        .unwrap();

    let handle = runtime.spawn(async move {
        debug!("Runtime handle");
    });

    runtime.block_on(handle).unwrap();

    // we no longer this runtime, pingora runtime will be used instead
    runtime.shutdown_background();
     */

    let dn_config = DnConfig::from_args(&app_config);
    let opt = dn_config.to_pingore_opt(&app_config);
    let config: ServerConf = dn_config.clone().into();

    let mut server = Server::new_with_opt_and_conf(opt, config);

    server.bootstrap();

    let dn_config_clone = dn_config.clone();
    for gateway_config in &dn_config_clone.gateways {
        let clone_gateway_config = gateway_config.clone();
        // clone_gateway_config.upstreams[0].
        let gateway_state: gateway::state::GatewayState = build_gateway_state(clone_gateway_config);
        let gateway_state_store = Arc::new(GatewayStateStore::new(gateway_state));
        let server_conf: ServerConf = dn_config_clone.clone().into();
        let service = build_http(gateway_state_store, Arc::new(server_conf));
        server.add_service(service);
    }

    /*
    let backends: LoadBalancer<selection::RoundRobin> =
        LoadBalancer::try_from_iter(["127.0.0.1:5002", "127.0.0.1:5102"]).unwrap();

    let mut proxy_service = http_proxy_service(
        &server.configuration,
        Proxy {
            load_balancer: Arc::new(backends),
        },
    );
    */
    /*
    let mut proxy_service = http_proxy_service(&server.configuration, Proxy {});
    proxy_service.add_tcp(&app_config.addr.as_str());
    server.add_service(proxy_service);
     */
    server.run_forever();
}
