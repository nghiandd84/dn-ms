use axum::Router;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_lookup_migrations::{Migrator, MigratorTrait};
use features_lookup_model::state::{LookupAppState, LookupCacheState};

use crate::{
    doc::ApiDoc,
    routes::{
        lookup_item::routes as lookup_item_routes,
        lookup_item_translation::routes as lookup_item_translation_routes,
        lookup_type::routes as lookup_type_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<LookupAppState, LookupCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async {
            Migrator::up((db).get_connection().as_ref(), None).await?;
            Ok(())
        }
    }

    fn routes(&self, app_state: &AppState<LookupAppState, LookupCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(lookup_type_routes(app_state))
            .merge(lookup_item_routes(app_state))
            .merge(lookup_item_translation_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new("LOOKUP".to_string(), Some("lookup".to_string()), true, true);

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Lookup app stopped");

    Ok(())
}
