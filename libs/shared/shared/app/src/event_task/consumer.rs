use futures_util::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::future::Future;
use tracing::{debug, error};

use crate::event_task::producer::{Producer, ProducerMessage};

pub struct ConsumerConfig {
    server: String,
    topic: String,
    group: String,
}

impl ConsumerConfig {
    pub fn from_env(server_env: String, topic_env: String, group: String) -> Self {
        let bootstrap_servers = std::env::var(&server_env)
            .map_err(|_| format!("${server_env} not set").into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "localhost:9092".to_string()
            });
        let consumer_topic = std::env::var(&topic_env)
            .map_err(|_| format!("${topic_env} not set").into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "notification_topic".to_string()
            });

        Self {
            server: bootstrap_servers,
            topic: consumer_topic,
            group,
        }
    }
}

pub async fn consumer_task<M, S, F, Fut>(
    config: ConsumerConfig,
    state: S,
    dlq_producer: Producer,
    dlq_key: String,
    handler: F,
) -> Result<(), Box<dyn std::error::Error + Send>>
where
    M: for<'de> Deserialize<'de> + std::fmt::Debug + Send + 'static,
    S: Clone + Send + 'static,
    F: Fn(M, S) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
{
    let bootstrap_server = config.server;
    let topic = config.topic;
    let group = config.group;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group.as_str())
        .set("bootstrap.servers", &bootstrap_server)
        .set("auto.offset.reset", "latest")
        .set("session.timeout.ms", "6000") // Example: longer session timeout
        .set("enable.auto.commit", "true")
        .set("allow.auto.create.topics", "true") // Allow Kafka to create topic if it doesn't exist
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topic])
        .expect("Can't subscribe to specified topic");

    // Wrap handler in Arc so it can be cloned into spawned tasks safely
    let handler = std::sync::Arc::new(handler);

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        debug!("Received message from Kafka");
        match message {
            Ok(borrow_message) => {
                let origin_message = match borrow_message.payload_view::<str>() {
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

                let message: M = match serde_json::from_str(origin_message) {
                    Ok(event) => event,
                    Err(e) => {
                        error!("Failed to deserialize message: {}", e);
                        continue;
                    }
                };
                debug!("Received Kafka event: {:?}", origin_message);
                let handler = handler.clone();
                let handler_state = state.clone();
                let handler_origin_message = origin_message.to_string();
                let handler_producer = dlq_producer.clone();
                let dlq_key_clone = dlq_key.clone();
                tokio::spawn(async move {
                    let result = (handler)(message, handler_state).await;
                    match result {
                        Ok(_) => {
                            debug!("Event handled successfully");
                        }
                        Err(e) => {
                            let error_message = e.to_string();
                            error!("Failed to handle event: {}", error_message);
                            debug!(
                                "Failed to handle event: {} and send message to DLQ topic",
                                error_message
                            );

                            let dlq_message = ProducerMessage {
                                payload: create_dlq_payload(
                                    handler_origin_message.to_string(),
                                    error_message,
                                )
                                .unwrap(),
                                key: Some(dlq_key_clone),
                            };
                            let result = handler_producer.send(&dlq_message).await;
                            if result.is_ok() {
                                debug!("Sent message to DLQ topic successfully");
                            } else {
                                error!(
                                    "Failed to send message to DLQ topic: {}",
                                    result.err().unwrap().reason
                                );
                            }
                        }
                    }
                });
            }
            Err(e) => {
                error!("Kafka error: {}", e)
            }
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct DlqMessage {
    origin_payload: String,
    error_msg: String,
}

fn create_dlq_payload(
    origin_payload: String,
    error_msg: String,
) -> Result<DlqMessage, serde_json::Error> {
    let report = DlqMessage {
        origin_payload,
        error_msg,
    };

    Ok(report)
}
