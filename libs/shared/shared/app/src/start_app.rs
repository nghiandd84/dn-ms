use axum::{middleware, Router};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, info};

use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use shared_shared_config::{db::Database, jwt::Jwt, mailer::Mailer};
use shared_shared_data_cache::cache::Cache;

use crate::config::AppConfig;
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
        app_state: Arc<AppState<C, T>>,
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

            let cache = Cache::<String, C>::new(cache_url.as_str(), cache_prefix.as_str())?;
            // match cache {
            //     Err(e) => {
            //         debug!("Error when connect redis {}", e);
            //     }
            //     Ok(data) => {
            //         debug!("Connect cache success")
            //     }
            // }
            // let cache = cache.unwrap();

            let app_state = AppState {
                conn: (&db).get_connection().clone(),
                cache,
                state,
            };

            // let my_app_state = app_state.clone();

            // let state = &app_state;

            let routes_all = Router::new()
                // .route("/healthchecker", get(health_checker_handler))
                // .route_layer(middleware::from_fn(mw_required_auth))
                .merge(self.routes(&app_state))
                .layer(middleware::map_response(main_response_mapper))
                .layer(middleware::from_fn(mw_ctx_resolver));
            // .layer(middleware::from_fn_with_state(
            //     my_app_state,
            //     mw_ctx_resolver,
            // ));
            self.custom_handler(Arc::new(app_state)).await?;
            let addr = format!("0.0.0.0:{port}");
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, routes_all.into_make_service())
                .await
                .unwrap();
            Ok(())
        }
    }
}
