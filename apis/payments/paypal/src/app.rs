use std::time::Duration;

use axum::Router;
use features_auth_remote::PermissionService;
use features_payments_core_remote::PaymentRemoteService;
use tokio::{spawn, time::interval};
use tracing::{debug, error};
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

use features_payments_paypal_migrations::{Migrator, MigratorTrait};
use features_payments_paypal_model::state::{PaymentsPaypalAppState, PaymentsPaypalCacheState};

use crate::{
    consumers::event_consumer::handler::handle_event_consumer_message,
    doc::ApiDoc,
    routes::{
        payment_flow::routes as payment_flow_routes,
        paypal_api_log::routes as api_log_routes,
        paypal_order::routes as order_routes,
        paypal_refund::routes as refund_routes,
        paypal_webhook_event::routes as webhook_event_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<PaymentsPaypalAppState, PaymentsPaypalCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let clone_app_state = app_state.clone();
        let mut perm_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let event_kafka_group = if instance_id.is_ok() {
            format!("paypal_for_event_{}", instance_id.unwrap())
        } else {
            "paypal_for_event".to_string()
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
                let service_key = "PAYMENT_PAYPAL".to_string();
                let mut interval = interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    let consul_client = get_consul_client().unwrap();
                    PermissionService::update_remote(&consul_client).await;
                    PaymentRemoteService::update_remote(&consul_client).await;
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

    fn routes(
        &self,
        app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>,
    ) -> Router {
        Router::new()
            .merge(order_routes(app_state))
            .merge(refund_routes(app_state))
            .merge(webhook_event_routes(app_state))
            .merge(api_log_routes(app_state))
            .merge(payment_flow_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "PAYMENT_PAYPAL".to_string(),
        Some("paypal".to_string()),
        true,
        true,
    );

    let client_id =
        std::env::var("PAYPAL_CLIENT_ID").expect("PAYPAL_CLIENT_ID must be set");
    let client_secret =
        std::env::var("PAYPAL_CLIENT_SECRET").expect("PAYPAL_CLIENT_SECRET must be set");
    let api_base = std::env::var("PAYPAL_API_BASE")
        .unwrap_or_else(|_| "https://api-m.sandbox.paypal.com".to_string());

    let http_client = reqwest::Client::new();

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app
        .start_app(Some(PaymentsPaypalAppState {
            http_client,
            client_id,
            client_secret,
            api_base,
        }))
        .await?;

    debug!("PayPal app stopped");

    Ok(())
}
