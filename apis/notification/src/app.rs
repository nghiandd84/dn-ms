use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

#[derive(Clone, Serialize, Deserialize)]
pub enum EmailTemplateCacheState {}

impl<'a> StartApp<EmailTemplateCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn migrate(
        &self,
        db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let _ = db;
        async { Ok(()) }
    }

    fn routes(&self, _app_state: &AppState<EmailTemplateCacheState>) -> Router {
        let all_routes = Router::new()
            // .merge(email_template_routes(app_state))
            // .merge(template_translation_routes(app_state))
            // .merge(template_placeholder_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "NOTIFICATION".to_string(),
        Some("notification".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    Ok(())
}
