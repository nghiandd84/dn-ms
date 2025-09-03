use futures_util::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tracing::{debug, error};

pub fn cusumer_task() -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> {
    async {
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

        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "notification_group")
            .set("bootstrap.servers", &kafka_bootstrap_servers)
            .set("enable.auto.commit", "true")
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
            match message {
                Ok(m) => {
                    if let Some(payload) = m.payload_view::<str>() {
                        debug!("Received message: {}", payload.unwrap());
                    }
                }
                Err(e) => error!("Kafka error: {}", e),
            }
        }
        Ok(())
    }
}
