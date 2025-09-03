use axum::{extract::Path, routing::get, Router};
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::{
    config::{AppConfig, DbConfig},
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use features_email_template_model::{
    state::{NotificationCacheState, NotificationState},
    types::new_clients,
};

use crate::websocket::handler::ws_handler;

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<NotificationCacheState, NotificationState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn migrate(
        &self,
        _db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async { Ok(()) }
    }

    fn routes(&self, app_state: &AppState<NotificationCacheState, NotificationState>) -> Router {
        let all_routes = Router::new()
            .route("/users/{user_id}", get(user_handler))
            .route("/ws", get(ws_handler))
            .with_state(app_state.clone());
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig {
        app_key: "NOTIFICATION_APP".to_string(),
        db_config: DbConfig {
            db_scheme: Some("notification_app".to_string()),
        },
        has_swagger: false,
    };

    let my_app = MyApp {
        config: &app_config,
    };

    let app_state = NotificationState::new(new_clients()); 

    my_app.start_app(Some(app_state)).await?;

    Ok(())
}

async fn user_handler(Path(user_id): Path<Uuid>) -> &'static str {
    debug!("User handler called {user_id}");
    "User handler response"
}
