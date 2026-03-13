use axum::Router;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_translation_migrations::{Migrator, MigratorTrait};
use features_translation_model::state::{TranslationAppState, TranslationCacheState};

use crate::{
    doc::ApiDoc,
    routes::{
        project::routes as project_routes, tag::routes as tag_routes,
        translation_key::routes as translation_key_routes,
        translation_version::routes as translation_version_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<TranslationAppState, TranslationCacheState> for MyApp<'a> {
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

    fn routes(&self, app_state: &AppState<TranslationAppState, TranslationCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(project_routes(app_state))
            .merge(translation_key_routes(app_state))
            .merge(tag_routes(app_state))
            .merge(translation_version_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "TRANSLATION".to_string(),
        Some("translation".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Translation app stopped");

    Ok(())
}
