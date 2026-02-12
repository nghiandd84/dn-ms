use axum::Router;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_event_migrations::{Migrator, MigratorTrait};
use features_event_model::state::{EventAppState, EventCacheState};

use crate::{
    doc::ApiDoc,
    routes::event::routes as event_routes,
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<EventAppState, EventCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async {
            Migrator::up((db).get_connection(), None).await?;
            Ok(())
        }
    }

    fn routes(&self, app_state: &AppState<EventAppState, EventCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(event_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "EVENT".to_string(),
        Some("event".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Event app stopped");

    Ok(())
}
