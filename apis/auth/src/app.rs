use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use features_auth_migrations::{Migrator, MigratorTrait};
use shared_shared_app::{
    config::{AppConfig, DbConfig},
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use crate::{
    doc::ApiDoc,
    routes::{
        client::routes as client_routes,
        scope::routes as scope_routes,
        login::routes as login_routes, profile::routes as profile_routes,
        register::routes as register_routes, role::routes as role_routes,
        user::routes as user_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp for MyApp<'a> {
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

    fn routes(&self, app_state: &AppState) -> Router {
        let all_routes = Router::new()
            .merge(scope_routes(app_state))
            .merge(client_routes(app_state))
            .merge(login_routes(app_state))
            .merge(register_routes(app_state))
            .merge(profile_routes(app_state))
            .merge(user_routes(app_state))
            .merge(role_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig {
        app_key: "AUTH".to_string(),
        db_config: DbConfig {
            db_scheme: Some("auth".to_string()),
        },
    };

    let my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app().await?;

    Ok(())
}
