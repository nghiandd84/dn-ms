use std::sync::{Arc, RwLock};

use axum::{extract::Path, routing::get, Router};
use tracing::{debug, error};
use uuid::Uuid;

use shared_shared_app::{
    config::AppConfig, discovery::get_consul_client, start_app::StartApp, state::AppState,
};
use shared_shared_config::db::Database;

use features_auth_remote::TokenService;
use features_email_template_model::{
    state::{NotificationCacheState, NotificationState},
    types::new_clients,
};

use crate::websocket::handler::message::ws_handler;

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<NotificationCacheState, Arc<RwLock<NotificationState>>> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &AppState<NotificationCacheState, Arc<RwLock<NotificationState>>>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let notification_state = app_state.state.clone().unwrap();
        async move {
            tokio::spawn(async move {
                debug!("Starting consumer task...");
                if let Err(e) = crate::consumer::task::cusumer_task(notification_state).await {
                    error!("Consumer task error: {}", e);
                    // Optionally handle the error here
                }
            });
            Ok(())
        }
    }

    fn migrate(
        &self,
        _db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async { Ok(()) }
    }

    fn routes(
        &self,
        app_state: &AppState<NotificationCacheState, Arc<RwLock<NotificationState>>>,
    ) -> Router {
        let all_routes = Router::new()
            .route("/ws", get(ws_handler))
            .with_state(app_state.clone());
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "NOTIFICATION_APP".to_string(),
        Some("autnotification_apph".to_string()),
        false,
        true,
    );

    let my_app = MyApp {
        config: &app_config,
    };

    let notification_state = Arc::new(RwLock::new(NotificationState::new(new_clients())));

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            // Place your code here to run every interval
            debug!("Interval task running...");
            let consul_client = get_consul_client().unwrap();
            TokenService::update_remote(&consul_client).await;
            let d = TokenService::get_instance().unwrap();
            debug!("Auth service instance: {:?}", d);
        }
    });

    my_app.start_app(Some(notification_state)).await?;

    Ok(())
}
