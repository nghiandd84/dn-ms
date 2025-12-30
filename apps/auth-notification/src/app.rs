use axum::Router;
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

use features_auth_model::state::AuthAppState;
use features_auth_stream::AuthMessage;
use features_email_template_remote::EmailTemplateService;

use crate::consumer::handler::handle_consumer_message;

struct MyApp<'a> {
    config: &'a AppConfig,
}

impl<'a> StartApp<AuthAppState> for MyApp<'a> {
    fn app_config(&self) -> &AppConfig {
        &self.config
    }

    fn custom_handler(
        &self,
        app_state: &mut AppState<AuthAppState>,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
        let auth_state = app_state.state.clone().unwrap();
        let app_key = self.config.app_key.clone();
        let instance_id = std::env::var("INSTANCE_ID");
        let kafka_group = if instance_id.is_ok() {
            format!("auth_notification_group_{}", instance_id.unwrap())
        } else {
            "auth_notification_group".to_string()
        };

        let consumer_config = ConsumerConfig::from_env(
            format!("{}_KAFKA_BOOTSTRAP_SERVERS", app_key),
            format!("{}_KAFKA_TOPIC", app_key),
            kafka_group,
        );

        async move {
            debug!("Starting custom handler for notification app...");
            let dlq_producer = Producer::from_config(ProducerConfig::from_env(
                "DLQ_KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "DLQ_KAFKA_TOPIC".to_string(),
            ))
            .await;
            tokio::spawn(async move {
                let res = consumer_task::<AuthMessage, AuthAppState, _, _>(
                    consumer_config,
                    auth_state,
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

    fn routes(&self, app_state: &AppState<AuthAppState>) -> Router {
        let all_routes = Router::new().with_state(app_state.clone());
        all_routes
    }
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::new(
        "AUTH_NOTIFICATION".to_string(),
        Some("auth_notification".to_string()),
        false,
        true,
    );

    let mut my_app = MyApp {
        config: &app_config,
    };

    let auth_state = AuthAppState::default();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            debug!("Interval task running...");
            let consul_client = get_consul_client().unwrap();
            EmailTemplateService::update_remote(&consul_client).await;
        }
    });

    my_app.start_app(Some(auth_state)).await?;

    Ok(())
}
