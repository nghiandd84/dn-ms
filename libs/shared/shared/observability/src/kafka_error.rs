use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;
use std::sync::Arc;
use tracing::{
    debug,
    field::{Field, Visit},
    Level,
};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub struct KafkaErrorSender {
    // Arc allows sharing the producer across threads (which tracing layers will be)
    producer: Arc<FutureProducer>,
    topic: String,
}

impl KafkaErrorSender {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000") // Optional: set a timeout
            .create()
            .expect("Producer creation error");

        KafkaErrorSender {
            producer: Arc::new(producer),
            topic: topic.to_string(),
        }
    }
}

struct MessageVisitor {
    message: String,
}

impl Visit for MessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

// Define a type alias for the Kafka sender's internal state
impl<S> Layer<S> for KafkaErrorSender
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    // The main function where events are handled
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();
        if metadata.level() == &Level::ERROR {
            let mut visitor = MessageVisitor {
                message: String::new(),
            };
            event.record(&mut visitor);
            let message = visitor.message;
            debug!("Captured error event: {}", message);

            let key = metadata.target().to_string();
            let file = metadata.file().unwrap_or("unknown");
            let line = metadata
                .line()
                .map(|l| l.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            let location = format!("{}:{}", file, line);

            let json_payload = json!({
                "location": location,
                "message": message.clone(),
            });
            let json_message = json_payload.to_string();

            let producer_clone = self.producer.clone();
            let topic_clone = self.topic.clone();
            let key_clone = key;

            // Spawn the async block to actually execute it
            let _handler = tokio::spawn(async move {
                let record: FutureRecord<String, String> = FutureRecord::to(&topic_clone)
                    .payload(&json_message)
                    .key(&key_clone);
                match producer_clone
                    .send(record, std::time::Duration::from_secs(0))
                    .await
                {
                    Ok((_partition, _offset)) => {
                        debug!("Error log sent to Kafka successfully");
                    }
                    Err(_e) => {
                        debug!("Failed to send error log to Kafka");
                    }
                }
            });
            // handler.abort(); // We don't need to wait for it to finish
        }
    }
}
