use axum::Router;
use tracing::{debug, error};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use shared_shared_app::{
    config::AppConfig,
    event_task::{
        consumer::{consumer_task, ConsumerConfig},
        producer::{Producer, ProducerConfig},
    },
    start_app::StartApp,
    state::AppState,
};
use shared_shared_config::db::Database;

use features_payments_stripe_migrations::{Migrator, MigratorTrait};
use features_payments_stripe_model::state::{PaymentsStripeAppState, PaymentsStripeCacheState};

use crate::{
    consumers::event_consumer::handler::handle_event_consumer_message,
    doc::ApiDoc,
    routes::{stripe_payment_intent::routes as payment_intent_routes, stripe_refund::routes as refund_routes, stripe_webhook_event::routes as webhook_event_routes, stripe_api_log::routes as api_log_routes},
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<PaymentsStripeAppState, PaymentsStripeCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<PaymentsStripeAppState, PaymentsStripeCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let clone_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let event_kafka_group = if instance_id.is_ok() {
            format!("stripe_for_event_{}", instance_id.unwrap())
        } else {
            "stripe_for_event".to_string()
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
                // let clone_arc_state = Arc::clone(&arc_state);
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

    fn routes(&self, app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(payment_intent_routes(app_state))
            .merge(refund_routes(app_state))
            .merge(webhook_event_routes(app_state))
            .merge(api_log_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "PAYMENT_STRIPE".to_string(),
        Some("stripe".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Stripe app stopped");

    Ok(())
}