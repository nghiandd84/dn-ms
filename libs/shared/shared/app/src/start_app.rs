use axum::routing::get;
use axum::{middleware, Router};
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tracing::{debug, info};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use shared_shared_config::db::Database;
use shared_shared_data_cache::cache::Cache;

use crate::config::AppConfig;
use crate::discovery::{deregister_service, get_consul_client, register_service};
use crate::health::health_checker_handler;
use crate::kafka_error::KafkaErrorSender;
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
        _app_state: &mut AppState<C, T>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async { Ok(()) }
    }

    fn app_config(&self) -> &AppConfig;
    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn start_app(
        &mut self,
        state: Option<T>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async move {
            dotenv().ok();

            // Set up tracing with Kafka error layer
            let kafka_server_env = "ERROR_KAFKA_BOOTSTRAP_SERVERS".to_string();
            let kafka_bootstrap_servers = std::env::var(&kafka_server_env)
                .map_err(|_| format!("${kafka_server_env} not set").into())
                .unwrap_or_else(|_e: String| "localhost:9092".to_string());
            let kafka_topic_env = "ERROR_KAFKA_TOPIC".to_string();
            let kafka_topic = std::env::var(&kafka_topic_env)
                .map_err(|_| format!("${kafka_topic_env} not set").into())
                .unwrap_or_else(|_e: String| "error_topic".to_string());
            let kafka_layer =
                KafkaErrorSender::new(kafka_bootstrap_servers.as_str(), kafka_topic.as_str());
            let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(log_level)) // drop debug/trace, keep info+
                .with(fmt::layer().compact().with_target(true))
                .with(kafka_layer)
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

            let consul_client = get_consul_client()?;
            let service_name = format!("{}_service", app_key.clone().to_lowercase());
            let service_port = port;
            let current_ip = local_ip_address::local_ip()
                .map(|ip| ip.to_string())
                .unwrap_or_else(|_| "127.0.0.1".to_string());
            let instance_id = format!("{}-{}-{}", service_name, current_ip, port);
            register_service(
                &consul_client,
                service_name.as_str(),
                instance_id.as_str(),
                current_ip.as_str(),
                service_port,
            )
            .await?;

            let db_connection = (&db).get_connection().clone();

            let mut app_state = AppState {
                conn: db_connection.clone(),
                cache,
                state,
                producer: Arc::new(Mutex::new(HashMap::new())),
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
            self.custom_handler(&mut app_state).await?;
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

            deregister_service(&consul_client, &service_name, instance_id.as_str()).await?;

            debug!("Stopped {} app", app_config.app_key);
            Ok(())
        }
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

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl+C received, shutting down");
        },
        _ = terminate => {
            info!("SIGTERM received, shutting down");
        },
    }
}
