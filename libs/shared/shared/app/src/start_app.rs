use axum::{middleware, routing::get, Router};
use tracing::debug;

use dotenv::dotenv;
use std::env;

use shared_shared_config::{cache::Cache, db::Database, jwt::Jwt, mailer::Mailer};

use crate::config::AppConfig;
use crate::health::health_checker_handler;
use crate::mapper::{main_response_mapper, mw_ctx_resolver};
use crate::state::AppState;

pub trait StartApp {
    // fn configure_service(cfg: &mut web::ServiceConfig);

    fn routes(&self, app_state: &AppState) -> Router;

    fn app_config(&self) -> &AppConfig;
    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>;

    fn start_app(
        &self,
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

            let port = env::var(format!("{}_PORT", app_config.app_key.clone()))
                .unwrap()
                .parse::<u16>()
                .unwrap();

            debug!(
                "{}",
                format!(
                    "Server is running on  {port} . Connect http://localhost:{port}/swagger-ui/",
                    port = port
                )
            );

            self.migrate(&db).await?;

            let app_state = AppState {
                conn: (&db).get_connection().clone(),
            };

            // let state = &app_state;

            let routes_all = Router::new()

                .route("/healthchecker", get(health_checker_handler))
                // .route_layer(middleware::from_fn(mw_required_auth))
                .merge(self.routes(&app_state))
                .layer(middleware::map_response(main_response_mapper))
                // .layer(middleware::from_fn(mw_ctx_resolver))
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    mw_ctx_resolver,
                ));

            let addr = format!("0.0.0.0:{port}");
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, routes_all.into_make_service())
                .await
                .unwrap();

            Ok(())
        }
    }
}
