use rdkafka::{producer::FutureProducer, util::Timeout, ClientConfig};
use serde::Serialize;
use tracing::{debug, error, instrument};

#[derive(Clone, Debug)]
pub struct ProducerConfig {
    pub kafka_server_env: String,
    pub kafka_topic_env: String,
}

impl Default for ProducerConfig {
    fn default() -> Self {
        Self {
            kafka_server_env: "KAFKA_BOOTSTRAP_SERVERS".to_string(),
            kafka_topic_env: "KAFKA_TOPIC".to_string(),
        }
    }
}

impl ProducerConfig {
    pub fn new(kafka_server_env: &str, kafka_topic_env: &str) -> Self {
        Self {
            kafka_server_env: kafka_server_env.to_string(),
            kafka_topic_env: kafka_topic_env.to_string(),
        }
    }
    pub fn from_env(server_env: String, topic_env: String) -> Self {
        let bootstrap_servers = std::env::var(&server_env)
            .expect(format!("producer kafka server variable ${} not set", server_env).as_str());

        let consumer_topic = std::env::var(&topic_env)
            .expect(format!("producer kafka topic variable ${} not set", topic_env).as_str());

        Self {
            kafka_server_env: bootstrap_servers,
            kafka_topic_env: consumer_topic,
        }
    }
}

pub struct ProducerMessage<T>
where
    T: Serialize,
{
    pub key: Option<String>,
    pub payload: T,
}

pub struct ProducerResult {
    pub partition: i32,
    pub offset: i64,
}

pub struct ProducerError {
    pub reason: String,
}

pub struct Producer {
    producer: FutureProducer,
    topic: String,
}

impl Clone for Producer {
    fn clone(&self) -> Self {
        Self {
            producer: self.producer.clone(),
            topic: self.topic.clone(),
        }
    }
}

impl Producer {
    pub async fn from_config(config: ProducerConfig) -> Self {
        let (producer, topic) = Self::get_producer_connection(config)
            .await
            .expect("Failed to create Kafka producer");
        Self { producer, topic }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    #[instrument(name = "send message", skip_all)]
    pub async fn send<T>(
        &self,
        message: &ProducerMessage<T>,
    ) -> Result<ProducerResult, ProducerError>
    where
        T: Serialize,
    {
        let payload_str = serde_json::to_string(&message.payload).map_err(|e| ProducerError {
            reason: format!("Serialization error: {}", e),
        })?;
        let current_span = tracing::Span::current();
        let topic = self.topic.clone();
        debug!("Sending Kafka message: {} via topic {}", payload_str, topic);
        current_span.record("message", &payload_str.as_str());
        current_span.record("topic", topic.as_str());
        let context = current_span.context();

        let mut headers = OwnedHeaders::new();
        // Inject current span context into Kafka headers
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut KafkaHeaderPropagator(&mut headers));
        });

        let key = message.key.clone().unwrap_or_default();
        let record = rdkafka::producer::FutureRecord::to(&self.topic)
            .payload(&payload_str)
            .headers(headers)
            .key(&key);

        match self.producer.send(record, Timeout::Never).await {
            Ok((partition, offset)) => Ok(ProducerResult { partition, offset }),
            Err((e, _)) => Err(ProducerError {
                reason: format!("Kafka send error: {}", e),
            }),
        }
    }

    fn get_producer_connection(
        config: ProducerConfig,
    ) -> impl std::future::Future<Output = Result<(FutureProducer, String), Box<dyn std::error::Error>>>
    {
        async move {
            let kafka_bootstrap_servers = config.kafka_server_env;

            let kafka_topic = config.kafka_topic_env;

            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &kafka_bootstrap_servers)
                .set("message.timeout.ms", "5000")
                .set("socket.connection.setup.timeout.ms", "10000")
                .set("socket.keepalive.enable", "true")
                .set("connections.max.idle.ms", "600000")
                .set("reconnect.backoff.ms", "100")
                .set("reconnect.backoff.max.ms", "10000")
                // Additional reliability settings
                .set("retries", "3") // Number of retries
                .set("retry.backoff.ms", "100") // Time between retries
                .set("acks", "all") // Strongest durability guarantee
                .create()
                .expect("Producer creation error");

            debug!(
                "Connect to kafka at servers: {} - topic: {}",
                kafka_bootstrap_servers, kafka_topic
            );

            Ok((producer, kafka_topic))
        }
    }
}

use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
};
use rdkafka::message::{Header, Headers, OwnedHeaders};
use tracing_opentelemetry::OpenTelemetrySpanExt;

struct KafkaHeaderPropagator<'a>(&'a mut OwnedHeaders);

impl<'a> Injector for KafkaHeaderPropagator<'a> {
    fn set(&mut self, key: &str, value: String) {
        let headers = std::mem::take(self.0);
        *self.0 = headers.insert(Header {
            key,
            value: Some(&value),
        });
    }
}
