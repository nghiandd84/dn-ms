mod kafka_error;
mod logging;
mod tracing;

use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;

use ::tracing::info;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, propagation::TraceContextPropagator, trace::SdkTracerProvider,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::kafka_error::KafkaErrorSender;
use crate::logging::init_otel_logger_provider;
use crate::tracing::init_otel_traces;

// https://dev.to/ciscoemerge/trace-through-a-kafka-cluster-with-rust-and-opentelemetry-2jln

pub fn init_log_trace_metric(
    service_name: String,
) -> Result<(SdkLoggerProvider, SdkTracerProvider), Box<dyn std::error::Error>> {
    // Set up tracing with Kafka error layer
    let kafka_server_env = "ERROR_KAFKA_BOOTSTRAP_SERVERS".to_string();
    let kafka_bootstrap_servers = std::env::var(&kafka_server_env)
        .map_err(|_| format!("${kafka_server_env} not set").into())
        .unwrap_or_else(|_e: String| "localhost:9092".to_string());
    let kafka_topic_env = "ERROR_KAFKA_TOPIC".to_string();
    let kafka_topic = std::env::var(&kafka_topic_env)
        .map_err(|_| format!("${kafka_topic_env} not set").into())
        .unwrap_or_else(|_e: String| "error_topic".to_string());

    // TODO check send error to kafka server
    let kafka_layer = KafkaErrorSender::new(kafka_bootstrap_servers.as_str(), kafka_topic.as_str());

    let logger_otel_provider = init_otel_logger_provider(service_name.clone());
    let logger_otel_layer = OpenTelemetryTracingBridge::new(&logger_otel_provider);

    let tracer_provider = init_otel_traces(service_name.clone());
    let tracer = tracer_provider.tracer(service_name.clone());
    let tracer_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    global::set_text_map_propagator(TraceContextPropagator::new());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracer_layer)
        // .with(log_file_layer)
        .with(kafka_layer)
        .with(logger_otel_layer)
        .init();

    info!("Tracing subscriber initialized");

    Ok((logger_otel_provider, tracer_provider))
}

pub fn init_otel_log_and_traces(
    service_name: String,
) -> Result<(SdkLoggerProvider, SdkTracerProvider), Box<dyn std::error::Error>> {
    let logger_otel_provider = init_otel_logger_provider(service_name.clone());
    // let logger_provider: &dyn opentelemetry::logs::LoggerProvider = &logger_otel_provider;
    // let logger = logger_otel_provider.logger("my.dioxus.logging.component");
    let logger_otel_layer = OpenTelemetryTracingBridge::new(&logger_otel_provider);

    let tracer_provider = init_otel_traces(service_name.clone());
    let tracer = tracer_provider.tracer(service_name.clone());
    let tracer_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let _subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracer_layer)
        .with(logger_otel_layer)
        .try_init();
    // Set the global Tracer Provider
    global::set_tracer_provider(tracer_provider.clone());
    info!("Tracing subscriber initialized");
    Ok((logger_otel_provider, tracer_provider))
}
