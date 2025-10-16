use futures_util::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use serde::Deserialize;
use tracing::{debug, error};

pub fn cusumer_task<E, F>(
    kafka_server_env: String,
    kafka_topic_env: String,
    kafka_group: String,
    handler: F,
) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>
where
    E: for<'de> Deserialize<'de> + std::fmt::Debug,
    F: Fn(E) + Send + 'static,
{
    async move {
        let kafka_bootstrap_servers = std::env::var(&kafka_server_env)
            .map_err(|_| {
                format!("${kafka_server_env}NOTIFICATION_APP_KAFKA_BOOTSTRAP_SERVERS not set")
                    .into()
            })
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "localhost:9092".to_string()
            });
        let kafka_topic = std::env::var(&kafka_topic_env)
            .map_err(|_| format!("${kafka_topic_env} not set").into())
            .unwrap_or_else(|e: String| {
                error!("{}", e);
                "notification_topic".to_string()
            });

        debug!(
            "Kafka bootstrap servers: {}, topic: {}, group: {}",
            kafka_bootstrap_servers, kafka_topic, kafka_group
        );

        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", kafka_group.as_str())
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

                    let event: E = match serde_json::from_str(message) {
                        Ok(event) => event,
                        Err(e) => {
                            error!("Failed to deserialize message: {}", e);
                            continue;
                        }
                    };
                    debug!("Received Kafka event: {:?}", event);
                    handler(event);
                }
                Err(e) => 
                {
                    // TODO implement retry logic
                    
                    // TODO implement dead letter queue
                    error!("Kafka error: {}", e)
                },
            }
        }
        Ok(())
    }
}
