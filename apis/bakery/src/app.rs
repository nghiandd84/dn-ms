use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use features_bakery_migrations::{Migrator, MigratorTrait};
use shared_shared_app::{
    config::{AppConfig, DbConfig},
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use crate::doc::ApiDoc;

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
            .merge(super::routes::baker::routes(app_state))
            .merge(super::routes::bakery::routes(app_state))
            .merge(super::routes::cake::routes(app_state))
            .merge(super::routes::cake_bakers::routes(app_state))
            .merge(super::routes::customer::routes(app_state))
            .merge(super::routes::order::routes(app_state))
            .merge(super::routes::lineitem::routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig {
        app_key: "BAKERY".to_string(),
        db_config: DbConfig {
            db_scheme: Some("bakery".to_string()),
        },
    };

    let my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app().await?;

    Ok(())
}
