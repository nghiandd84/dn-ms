use std::time::Duration;

use axum::{middleware::from_fn, Router};
use features_auth_remote::PermissionService;
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

use features_wallet_migrations::{Migrator, MigratorTrait};
use features_wallet_model::state::{WalletAppState, WalletCacheState};

use crate::{
    consumers::payment_core_consumer::handler::handle_payment_core_message,
    doc::ApiDoc,
    middleware::idempotency_tracking_middleware,
    routes::{
        idempotency::routes as idempotency_routes, p2p_transfer::routes as p2p_transfer_routes,
        top_up_transaction::routes as top_up_transaction_routes,
        transaction::routes as transaction_routes, wallet::routes as wallet_routes,
        withdrawal::routes as withdrawal_routes,
    },
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<WalletAppState, WalletCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<WalletAppState, WalletCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let mut clone_app_state = app_state.clone();
        let consumer_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let payment_core_kafka_group = if let Ok(ref id) = instance_id {
            format!("wallet_for_payment_core_{}", id)
        } else {
            "wallet_for_payment_core".to_string()
        };

        let consumer_config = ConsumerConfig::from_env(
            format!("{}_CONSUMER_PAYMENT_CORE_KAFKA_BOOTSTRAP_SERVERS", app_key),
            format!("{}_CONSUMER_PAYMENT_CORE_KAFKA_TOPIC", app_key),
            payment_core_kafka_group,
        );

        async move {
            // Spawn payment-core consumer
            let dlq_producer = Producer::from_config(ProducerConfig::from_env(
                "DLQ_KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "DLQ_KAFKA_TOPIC".to_string(),
            ))
            .await;
            let dlq_app_key = app_key.clone();
            tokio::spawn(async move {
                if let Err(e) = consumer_task(
                    consumer_config,
                    consumer_app_state,
                    dlq_producer,
                    dlq_app_key,
                    handle_payment_core_message,
                )
                .await
                {
                    error!("Error in payment-core consumer task: {:?}", e);
                }
            });

            // Permission sync loop
            spawn(async move {
                let service_key = "WALLET".to_string();
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

    fn routes(&self, app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(wallet_routes(app_state))
            .merge(transaction_routes(app_state))
            .merge(top_up_transaction_routes(app_state))
            .merge(p2p_transfer_routes(app_state))
            .merge(withdrawal_routes(app_state))
            .merge(idempotency_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(from_fn(idempotency_tracking_middleware));

        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new("WALLET".to_string(), Some("wallet".to_string()), true, true);

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Wallet app stopped");

    Ok(())
}
