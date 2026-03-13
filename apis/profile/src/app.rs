use axum::Router;
use features_profiles_model::state::{ProfileAppState, ProfileCacheState};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_profiles_migrations::{Migrator, MigratorTrait};

use crate::{
    doc::ApiDoc,
    routes::{
        profile::routes as profile_routes, social_link::routes as social_link_routes,
        user_preference::routes as user_preference_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<ProfileAppState, ProfileCacheState> for MyApp<'a> {
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

    fn routes(&self, app_state: &AppState<ProfileAppState, ProfileCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(profile_routes(app_state))
            .merge(user_preference_routes(app_state))
            .merge(social_link_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "PROFILE".to_string(),
        Some("profile".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Profile app stopped");

    Ok(())
}
