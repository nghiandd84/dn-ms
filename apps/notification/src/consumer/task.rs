use futures_util::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::sync::{Arc, RwLock};
use tracing::{debug, error};

use features_email_template_model::state::NotificationState;

use crate::consumer::event::KafkaEvent;
use crate::consumer::handler::handler_event;

pub fn cusumer_task(
    notification_state: Arc<RwLock<NotificationState>>,
) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
    async move {
        debug!("Starting Kafka consumer task...");
        let kafka_bootstrap_servers = std::env::var("NOTIFICATION_APP_KAFKA_BOOTSTRAP_SERVERS")
            .map_err(|_| ("NOTIFICATION_APP_KAFKA_BOOTSTRAP_SERVERS not set").into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "localhost:9092".to_string()
            });
        let kafka_topic = std::env::var("NOTIFICATION_APP_KAFKA_TOPIC")
            .map_err(|_| "NOTIFICATION_APP_KAFKA_TOPIC not set".into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "notification_topic".to_string()
            });

        let instance_id = std::env::var("INSTANCE_ID")
            .map_err(|_| ("INSTANCE_ID not set").into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "0".to_string()
            });
        // Each instance should have its own group id to receive all messages
        let notification_group = format!("notification_group_{}", instance_id);
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", notification_group.as_str())
            .set("bootstrap.servers", &kafka_bootstrap_servers)
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "6000") // Example: longer session timeout
            .set("enable.auto.commit", "true")
            .set("allow.auto.create.topics", "true") // Allow Kafka to create topic if it doesn't exist
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&kafka_topic])
            .expect("Can't subscribe to specified topic");

        debug!("Subscribed to topic: {}", kafka_topic);

        let mut message_stream = consumer.stream();
        while let Some(message) = message_stream.next().await {
            debug!("Received message from Kafka");
            match message {
                Ok(m) => {
                    let message = match m.payload_view::<str>() {
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            error!("Error while deserializing message payload: {:?}", e);
                            continue;
                        }
                        None => {
                            error!("No payload in message");
                            continue;
                        }
                    };

                    let event: KafkaEvent = match serde_json::from_str(message) {
                        Ok(event) => event,
                        Err(e) => {
                            error!("Failed to deserialize message: {}", e);
                            continue;
                        }
                    };
                    debug!("Received Kafka event: {:?}", event);
                    handler_event(event, &notification_state).await;
                }
                Err(e) => error!("Kafka error: {}", e),
            }
        }
        Ok(())
    }
}
