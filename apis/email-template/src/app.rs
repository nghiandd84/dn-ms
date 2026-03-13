use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{config::AppConfig, start_app::StartApp, state::AppState};
use shared_shared_config::db::Database;

use features_email_template_migrations::{Migrator, MigratorTrait};
use features_email_template_model::state::EmailTemplateCacheState;

use crate::{
    doc::ApiDoc,
    routes::{
        email_template::routes as email_template_routes,
        template_placeholder::routes as template_placeholder_routes,
        template_translation::routes as template_translation_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<EmailTemplateCacheState> for MyApp<'a> {
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

    fn routes(&self, app_state: &AppState<EmailTemplateCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(email_template_routes(app_state))
            .merge(template_translation_routes(app_state))
            .merge(template_placeholder_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "EMAIL_TEMPLATE".to_string(),
        Some("email_template".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    Ok(())
}
