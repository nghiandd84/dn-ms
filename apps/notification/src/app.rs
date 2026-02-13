use axum::{routing::get, Router};
use features_notification_stream::message::NotificationMessage;
use std::sync::{Arc, RwLock};
use tracing::{debug, error};

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

use features_auth_remote::TokenService;
use features_notification_model::{
    state::{NotificationCacheState, NotificationState},
    types::new_clients,
};

use crate::{consumer::handler::handle_consumer_message, websocket::handler::message::ws_handler};

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<Arc<RwLock<NotificationState>>, NotificationCacheState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<Arc<RwLock<NotificationState>>, NotificationCacheState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let notification_state = app_state.state.clone().unwrap();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let kafka_group = if instance_id.is_ok() {
            format!("notification_{}", instance_id.unwrap())
        } else {
            "notification".to_string()
        };

        let consumer_config = ConsumerConfig::from_env(
            format!("{}_KAFKA_BOOTSTRAP_SERVERS", app_key),
            format!("{}_KAFKA_TOPIC", app_key),
            kafka_group,
        );

        async move {
            let dlq_producer = Producer::from_config(ProducerConfig::from_env(
                "DLQ_KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "DLQ_KAFKA_TOPIC".to_string(),
            ))
            .await;
            tokio::spawn(async move {
                let res =
                    consumer_task::<NotificationMessage, Arc<RwLock<NotificationState>>, _, _>(
                        consumer_config,
                        notification_state,
                        dlq_producer,
                        app_key.clone(),
                        handle_consumer_message,
                    )
                    .await;

                if let Err(e) = res {
                    error!("Consumer task exited with error: {}", e);
                }
            });

            Ok(())
        }
    }

    fn migrate(
        &self,
        _db: &Database,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        async { Ok(()) }
    }

    fn routes(
        &self,
        app_state: &AppState<Arc<RwLock<NotificationState>>, NotificationCacheState>,
    ) -> Router {
        let all_routes = Router::new()
            .route("/ws", get(ws_handler))
            .with_state(app_state.clone());
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "NOTIFICATION_APP".to_string(),
        Some("autnotification_apph".to_string()),
        false,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    let notification_state = Arc::new(RwLock::new(NotificationState::new(new_clients())));

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            debug!("Interval task running...");
            let consul_client = get_consul_client().unwrap();
            TokenService::update_remote(&consul_client).await;
        }
    });

    my_app.start_app(Some(notification_state)).await?;

    Ok(())
}
