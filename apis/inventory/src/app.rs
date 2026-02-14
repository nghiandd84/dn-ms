use std::{
    clone,
    sync::{Arc, RwLock},
};

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

use features_inventory_migrations::{Migrator, MigratorTrait};
use features_inventory_model::state::{InventoryAppState, InventoryCacheState};

use crate::{
    consumers::event_consumer::handler::handle_event_consumer_message,
    doc::ApiDoc,
    routes::{reservation::routes as reservation_routes, seat::routes as seat_routes},
};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<InventoryAppState, InventoryCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<InventoryAppState, InventoryCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let clone_app_state = app_state.clone();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let event_kafka_group = if instance_id.is_ok() {
            format!("inventory_for_event_{}", instance_id.unwrap())
        } else {
            "inventory_for_event".to_string()
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
            Migrator::up((db).get_connection(), None).await?;
            Ok(())
        }
    }

    fn routes(&self, app_state: &AppState<InventoryAppState, InventoryCacheState>) -> Router {
        let all_routes = Router::new()
            .merge(seat_routes(app_state))
            .merge(reservation_routes(app_state))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "INVENTORY".to_string(),
        Some("inventory".to_string()),
        true,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    my_app.start_app(None).await?;

    debug!("Inventory app stopped");

    Ok(())
}
