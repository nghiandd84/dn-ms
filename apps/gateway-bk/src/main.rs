mod config;
mod error;
mod gateway;
mod proxy;
mod qe;
mod shared;

use std::{
    mem::take,
    sync::{Arc, Mutex},
};

use clap::Parser;
use config::{DakiaArgs, DakiaConfig};
use error::DakiaError;
use gateway::state::build_gateway_state;
use gateway::state::GatewayStateStore;
use gateway::HttpGateway;

use pingora::server::{configuration::ServerConf, Server};
use shared::{common::get_dakia_ascii_art, dakia_state::DAKIA_STATE_STORE};

use proxy::http::Proxy;
use shared::into::IntoRef;
use tokio::runtime::Builder;
use tracing::debug;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    println!("{}", get_dakia_ascii_art());

    let dakia_args = DakiaArgs::parse();
    debug!("dakia_args{:?}", dakia_args);

    let dakia_config = DakiaConfig::from_args(dakia_args.clone()).unwrap();

    // debug!("dakia_config {:?}", dakia_config);
    DAKIA_STATE_STORE
        .store_dakia_config(dakia_config.clone())
        .unwrap();

    process_args(&dakia_args, &dakia_config).unwrap();

    debug!("process_args {:?}", dakia_config);


    let runtime = Builder::new_current_thread()
        .build()
        // if there is any error, just panic
        .unwrap();

    // TODO: add support for TCP, WebSocket and gRPC gateway
    let gateways: Arc<Mutex<Vec<HttpGateway>>> = Arc::new(Mutex::new(vec![]));

    // clone data for passing to the tokio runtime
    let gateways_cloned = gateways.clone();
    let dakia_config_cloned = dakia_config.clone();

    let handle = runtime.spawn(async move {
        let mut gateway_state_stores: Vec<Arc<GatewayStateStore>> = vec![];

        for gateway_config in &dakia_config_cloned.gateways {
            debug!("gateway_config {:?}", gateway_config);
            let cloned_gateway_config = gateway_config.clone();

            // dakia can not work without state, so unwrap is not a problem
            let gateway_state = build_gateway_state(cloned_gateway_config, dakia_config.version)
                .await
                .unwrap();
            let gateway_state_store = Arc::new(GatewayStateStore::new(gateway_state));
            let server_conf: ServerConf = dakia_config_cloned.into_ref();

            let gateway = gateway::build_http(gateway_state_store.clone(), Arc::new(server_conf))
                .await
                .unwrap();

            // rust mutex guard does not work properly across tokio await, so creating lock guard after await in each loop
            let mut gateway_vector_guard = gateways_cloned.lock().unwrap();
            gateway_vector_guard.push(gateway);
            gateway_state_stores.push(gateway_state_store);
        }

        DAKIA_STATE_STORE
            .store_gateway_state_stores(gateway_state_stores)
            .unwrap();
    });

    runtime.block_on(handle).unwrap();

    // we no longer this runtime, pingora runtime will be used instead
    runtime.shutdown_background();

    let mut server = Server::new_with_opt_and_conf(
        dakia_config.to_pingore_opt(&dakia_args),
        dakia_config.into_ref(),
    );

    server.bootstrap();

    let mut gateway_vector_guard = gateways.lock().unwrap();

    // take ownership of vector to pass owned value inside add_service
    let proxy_vector = take(&mut *gateway_vector_guard);

    for gateway in proxy_vector.into_iter() {
        server.add_service(gateway);
    }
    debug!("Server configuration {:?}", server.configuration);
    server.run_forever();
}

fn process_args(args: &DakiaArgs, dakia_config: &DakiaConfig) -> Result<(), Box<DakiaError>> {
    if args.version {
        debug!("Process version");
        // version will be printed along with dakia art in the very beginning, so just exist from here
        shared::common::exit();
    }

    if args.reload {
        debug!("Process reload");
        todo!();
    }

    if args.debug {
        debug!("Process debug");
        println!("{:?}", dakia_config);
        shared::common::exit();
    }

    if args.test {
        debug!("Process test");
        todo!();
    }
    // TODO: use kill -HUP pid
    debug!("Proccess gateway");
    Ok(())
}
