use axum::Router;
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig,
    event_task::producer::{Producer, ProducerConfig},
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use features_merchant_migrations::{Migrator, MigratorTrait};
use features_merchant_model::state::{MerchantAppState, MerchantCacheState};
use features_merchant_stream::PRODUCER_KEY;

use crate::{doc::ApiDoc, routes::merchant::routes as merchant_routes};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<MerchantAppState, MerchantCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<MerchantAppState, MerchantCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async move {
            let app_key = self.config.app_key.clone();

            let kafka_server_env = format!("{}_KAFKA_BOOTSTRAP_SERVERS", app_key);
            let kafka_topic_env = format!("{}_KAFKA_TOPIC", app_key);

            let producer_config =
                ProducerConfig::from_env(kafka_server_env.clone(), kafka_topic_env.clone());
            debug!("Creating Kafka producer with config {:?}", producer_config);
            let producer = Producer::from_config(producer_config).await;
            app_state.set_producer(PRODUCER_KEY.to_string(), producer);

            Ok(())
        }
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

    fn routes(&self, app_state: &AppState<MerchantAppState, MerchantCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(merchant_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "MERCHANT".to_string(),
        Some("merchant".to_string()),
        true,
        true,
    );

    let mut app = MyApp { config: &app_config };
    app.start_app(Some(MerchantAppState::default())).await?;

    debug!("Merchant app stopped");
    Ok(())
}
