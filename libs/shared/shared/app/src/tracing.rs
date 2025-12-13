use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, propagation::TraceContextPropagator, trace::SdkTracerProvider,
    Resource,
};
use std::env;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::kafka_error::KafkaErrorSender;

// https://dev.to/ciscoemerge/trace-through-a-kafka-cluster-with-rust-and-opentelemetry-2jln

pub fn init_tracing_log(
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

    let default_port = 6101;
    let port = env::var(format!("{}_PORT", service_name.clone()))
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| default_port);
    let log_dir = env::var("RUST_LOG_DIRECTORY").unwrap_or_else(|_| "./logs".to_string());
    let log_dir = format!("{}/{}", log_dir, service_name.to_lowercase());
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    let log_file_name = format!("{}_{}.log", service_name, port).to_lowercase();
    let log_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix(log_file_name)
        .build(log_dir)
        .expect("Failed to create log appender");

    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_appender)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_ansi(true);

    let tracer_provider = init_otel_traces(service_name.clone());
    let tracer = tracer_provider.tracer(service_name.clone());
    let tracer_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // let sdk_tracer = tracer_provider.tracer(service_name.as_str());
    global::set_text_map_propagator(TraceContextPropagator::new());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracer_layer)
        .with(log_layer)
        .with(
            if std::env::var("ENVIRONMENT").unwrap_or_default() != "production" {
                Some(tracing_subscriber::fmt::layer())
            } else {
                None
            },
        )
        .with(kafka_layer)
        .with(logger_otel_layer)
        .init();

    info!("Tracing subscriber initialized");

    Ok((logger_otel_provider, tracer_provider))
}

fn init_otel_logger_provider(service_name: String) -> SdkLoggerProvider {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        // .with_tonic()
        .with_http()
        .build()
        .expect("Failed to create log exporter");
    let logger_provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .with_batch_exporter(exporter)
        .build();

    logger_provider
}

fn init_otel_traces(service_name: String) -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_http()
        // .with_protocol(Protocol::HttpBinary) //can be changed to `Protocol::HttpJson` to export in JSON format
        .build()
        .expect("Failed to create trace exporter");

    let resouce = Resource::builder()
        .with_service_name(service_name.clone())
        .with_attribute(KeyValue::new("service.version", "1.0"))
        .build();

    let trace_provider = SdkTracerProvider::builder()
        // .with_span_processor
        .with_batch_exporter(exporter)
        .with_resource(resouce)
        .build();

    trace_provider
}
