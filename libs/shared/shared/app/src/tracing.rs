use opentelemetry::trace::TracerProvider;
use std::env;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};

use crate::kafka_error::KafkaErrorSender;

// https://dev.to/ciscoemerge/trace-through-a-kafka-cluster-with-rust-and-opentelemetry-2jln

pub fn init_tracing_log(service_name: String) -> anyhow::Result<()> {
    // Set up tracing with Kafka error layer
    let kafka_server_env = "ERROR_KAFKA_BOOTSTRAP_SERVERS".to_string();
    let kafka_bootstrap_servers = std::env::var(&kafka_server_env)
        .map_err(|_| format!("${kafka_server_env} not set").into())
        .unwrap_or_else(|_e: String| "localhost:9092".to_string());
    let kafka_topic_env = "ERROR_KAFKA_TOPIC".to_string();
    let kafka_topic = std::env::var(&kafka_topic_env)
        .map_err(|_| format!("${kafka_topic_env} not set").into())
        .unwrap_or_else(|_e: String| "error_topic".to_string());
    let kafka_layer = KafkaErrorSender::new(kafka_bootstrap_servers.as_str(), kafka_topic.as_str());
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    // let fmt_layer = tracing_subscriber::fmt::layer();

    // Opentelemetry tracing layer can be added here if needed
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");

    let tracer = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build()
        .tracer(service_name.clone());
    // let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(tracing_subscriber::EnvFilter::new(log_level)) // drop debug/trace, keep info+
        .with(kafka_layer)
        // .with(otel_layer)
        .with(fmt::layer().compact().with_target(true));
        // .with(tracing_opentelemetry::layer().with_tracer(tracer));
    subscriber.init();

    info!("Tracing subscriber initialized");

    Ok(())
}

// End
