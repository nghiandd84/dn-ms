use std::time::Duration;

use axum::Router;
use features_auth_remote::PermissionService;
use tokio::{spawn, time::interval};
use tracing::error;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig,
    discovery::get_consul_client,
    event_task::{
        consumer::{consumer_task, ConsumerConfig},
        producer::{Producer, ProducerConfig},
    },
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use features_payments_core_migrations::{Migrator, MigratorTrait};
use features_payments_core_model::state::{PaymentsCoreAppState, PaymentsCoreCacheState};

use crate::{
    consumers::event_consumer::handler::handle_event_consumer_message,
    doc::ApiDoc,
    routes::{
        payment::routes as payment_routes, payment_attempt::routes as payment_attempt_routes,
        payment_method::routes as payment_method_routes,
        payment_method_limit::routes as payment_method_limit_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<PaymentsCoreAppState, PaymentsCoreCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<PaymentsCoreAppState, PaymentsCoreCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let clone_app_state = app_state.clone();
        let mut perm_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let event_kafka_group = if instance_id.is_ok() {
            format!("payment_core_for_event_{}", instance_id.unwrap())
        } else {
            "payment_core_for_event".to_string()
        };

        let consumer_config = ConsumerConfig::from_env(
            format!("{}_CONSUMER_EVENT_KAFKA_BOOTSTRAP_SERVERS", app_key),
            format!("{}_CONSUMER_EVENT_KAFKA_TOPIC", app_key),
            event_kafka_group,
        );
        async move {
            let dlq_producer = Producer::from_config(ProducerConfig::from_env(
                "DLQ_KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "DLQ_KAFKA_TOPIC".to_string(),
            ))
            .await;
            tokio::spawn(async move {
                if let Err(e) = consumer_task(
                    consumer_config,
                    clone_app_state,
                    dlq_producer,
                    app_key.clone(),
                    handle_event_consumer_message,
                )
                .await
                {
                    error!("Error in consumer task: {:?}", e);
                }
            });

            spawn(async move {
                let service_key = "PAYMENT_CORE".to_string();
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
                        perm_app_state.set_permission_map(role_name, mask_permissions);
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

    fn routes(&self, app_state: &AppState<PaymentsCoreAppState, PaymentsCoreCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(payment_routes(app_state))
            .merge(payment_attempt_routes(app_state))
            .merge(payment_method_routes(app_state))
            .merge(payment_method_limit_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "PAYMENT_CORE".to_string(),
        Some("payment".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    Ok(())
}
