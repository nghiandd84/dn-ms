use std::time::Duration;

use axum::Router;
use features_auth_remote::PermissionService;
use tokio::{spawn, time::interval};
use tracing::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig,
    discovery::get_consul_client,
    event_task::producer::{Producer, ProducerConfig},
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use features_merchant_migrations::{Migrator, MigratorTrait};
use features_merchant_model::state::{MerchantAppState, MerchantCacheState};
use features_merchant_stream::PRODUCER_KEY;

use crate::{
    doc::ApiDoc,
    routes::{
        api_key::routes as api_key_routes, merchant::routes as merchant_routes,
        webhook::routes as webhook_routes,
    },
};

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
        let mut clone_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        async move {
            let kafka_server_env = format!("{}_KAFKA_BOOTSTRAP_SERVERS", app_key);
            let kafka_topic_env = format!("{}_KAFKA_TOPIC", app_key);

            let producer_config =
                ProducerConfig::from_env(kafka_server_env.clone(), kafka_topic_env.clone());
            debug!("Creating Kafka producer with config {:?}", producer_config);
            let producer = Producer::from_config(producer_config).await;
            clone_app_state.set_producer(PRODUCER_KEY.to_string(), producer);

            spawn(async move {
                let service_key = "MERCHANT".to_string();
                let mut interval = interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    let consul_client = get_consul_client().unwrap();
                    PermissionService::update_remote(&consul_client).await;
                    let all_permissions =
                        PermissionService::get_roles_by_service_name(service_key.clone()).await;
                    for (role_name, permissions) in all_permissions {
                        let mask_permissions = permissions
                            .iter()
                            .map(|perm| {
                                (
                                    perm.resource.clone().unwrap_or_default(),
                                    perm.mask.unwrap_or(0) as u32,
                                )
                            })
                            .collect();
                        clone_app_state.set_permission_map(role_name, mask_permissions);
                    }
                }
            });
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
            .merge(api_key_routes(app_state))
            .merge(webhook_routes(app_state))
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

    let mut app = MyApp {
        config: &app_config,
    };
    app.start_app(Some(MerchantAppState::default())).await?;

    debug!("Merchant app stopped");
    Ok(())
}
