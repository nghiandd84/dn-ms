use axum::routing::get;
use axum::{middleware, Router};
use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::service;
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use tokio::signal;
use tracing::{debug, info};
use utoipa::openapi::info;

use shared_shared_config::{db::Database, jwt::Jwt, mailer::Mailer};
use shared_shared_data_cache::cache::Cache;

use crate::config::AppConfig;
use crate::health::health_checker_handler;
use crate::mapper::{main_response_mapper, mw_ctx_resolver};
use crate::state::AppState;

pub trait StartApp<C, T = ()>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    fn routes(&self, app_state: &AppState<C, T>) -> Router;

    fn custom_handler(
        &self,
        app_state: &AppState<C, T>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async { Ok(()) }
    }

    fn app_config(&self) -> &AppConfig;
    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn start_app(
        &self,
        state: Option<T>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async {
            dotenv().ok();
            env::set_var("RUST_LOG", "debug");

            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_test_writer()
                .init();
            let app_config = self.app_config();
            let app_key = app_config.app_key.clone();
            let db_scheme = app_config
                .db_config
                .db_scheme
                .clone()
                .unwrap_or(app_key.clone().to_string().to_lowercase());

            let mut db = Database::new(
                Some(format!("{}_DATABASE_URL", app_key.clone())),
                Some(db_scheme),
            );

            db.connect().await;

            let default_port = 6101;
            let port = env::var(format!("{}_PORT", app_config.app_key.clone()))
                .unwrap_or_else(|_| default_port.to_string())
                .parse::<u16>()
                .unwrap_or_else(|_| default_port);

            info!("Starting {} app on port {}", app_config.app_key, port);

            if app_config.has_swagger {
                debug!(
                    "{}",
                    format!(
                    "Server is running on  {port} . Connect http://localhost:{port}/swagger-ui/",
                    port = port
                )
                );
            }

            self.migrate(&db).await?;

            // Cache
            let default_cache_url: &str = "redis://127.0.0.1/";
            let cache_prefix = app_key.clone();
            let cache_url = env::var(format!("{}_REDIS_URL", app_config.app_key.clone()))
                .unwrap_or_else(|_| default_cache_url.to_string());
            debug!(
                "Connect cache with url {} and prefix {}",
                cache_url, cache_prefix
            );

            let cache = Cache::<String, C>::new(cache_url.as_str(), cache_prefix.as_str())
                .expect("Failed to connect to redis cache");

            let client = self.initialize_consul_client()?;
            // Register service to Consul
            let service_name = format!("{}_service", app_key.clone().to_lowercase());
            let service_port = port;
            let current_ip = local_ip_address::local_ip()
                .map(|ip| ip.to_string())
                .unwrap_or_else(|_| "127.0.0.1".to_string());
            debug!("Current service IP: {}", current_ip);
            let health_check = AgentServiceCheckBuilder::default()
                .name("http-health-check")
                .interval("10s") // Check every 10 seconds
                .http(format!(
                    "http://{}:{}/healthchecker",
                    current_ip, service_port
                ))
                .status("passing")
                .build()
                .unwrap();
            let instance_id = format!("{}-{}-{}", service_name, current_ip, port);
            // Send the registration request to the Consul agent
            service::register(
                &client,
                service_name.as_str(),
                Some(
                    RegisterServiceRequest::builder()
                        .id(&instance_id)
                        .address(format!("http://{}", current_ip))
                        .port(service_port)
                        .check(health_check),
                ),
            )
            .await?;

            println!("Service '{}' registered successfully!", service_name);

            let db_connection = (&db).get_connection().clone();

            let app_state = AppState {
                conn: db_connection.clone(),
                cache,
                state,
            };

            let routes_all = Router::new()
                .route("/healthchecker", get(health_checker_handler))
                // .route_layer(middleware::from_fn(mw_required_auth))
                .merge(self.routes(&app_state))
                .layer(middleware::map_response(main_response_mapper))
                .layer(middleware::from_fn(mw_ctx_resolver));
            // .layer(middleware::from_fn_with_state(
            //     my_app_state,
            //     mw_ctx_resolver,
            // ));
            self.custom_handler(&app_state).await?;
            let addr = format!("0.0.0.0:{port}");
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            debug!("Listener created");
            axum::serve(listener, routes_all.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();

            info!("Gracefully shutting down");
            // Close DB connection
            db_connection.close().await?;
            info!("Database connection closed");

            // TODO disconnect Cache
            info!("Cache connection closed");

            // Deregister service
            let deregister = service::deregister(&client, service_name.as_str(), None).await;
            match deregister {
                Ok(_) => info!("Service '{}' deregistered successfully!", service_name),
                Err(e) => debug!("Failed to deregister service '{}': {}", service_name, e),
            }

            debug!("Stopped {} app", app_config.app_key);
            Ok(())
        }
    }

    fn initialize_consul_client(self: &Self) -> Result<ConsulClient, Box<dyn std::error::Error>> {
        let default_consul_url = "http://127.0.0.1:8500";
        let consul_url = env::var(format!("CONSUL_URL"))
            .unwrap_or_else(|_| default_consul_url.to_string())
            .parse::<String>()
            .unwrap_or_else(|_| default_consul_url.to_string());
        // ... client creation logic ...
        let settings = ConsulClientSettingsBuilder::default()
            .address(consul_url.clone())
            .build()?;

        // 2. Create the asynchronous Consul client
        let client = ConsulClient::new(settings)?;

        info!("Consul client initialized successfully at {}", consul_url);

        Ok(client)
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        info!("Press Ctrl+C to shut down");
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        info!("Press SIGTERM to shut down");
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl+C received, shutting down");
        },
        _ = terminate => {
            info!("SIGTERM received, shutting down");
        },
    }
}
