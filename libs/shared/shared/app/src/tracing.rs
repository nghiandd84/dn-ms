use opentelemetry::trace::{FutureExt, TracerProvider};
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};
use std::env;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{fmt::format::FmtSpan, Registry};

use crate::kafka_error::KafkaErrorSender;

// https://dev.to/ciscoemerge/trace-through-a-kafka-cluster-with-rust-and-opentelemetry-2jln

pub fn init_tracing_log(service_name: String) -> Result<(), Box<dyn std::error::Error>> {
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
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    // let fmt_layer = tracing_subscriber::fmt::layer();

    // Opentelemetry tracing layer can be added here if needed
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        // .with_tonic()
        .with_http()
        .build()
        .expect("Failed to create span exporter");
    let exporter = opentelemetry_stdout::LogExporter::default();

    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("log-appender-tracing-example")
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();
    
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider);

    /*
    let tracer = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)

        // .with_batch_exporter(otlp_exporter)

        .build()
        .tracer(service_name.clone());
    */
    // Configure the OTLP exporter (defaults to sending to localhost:4317)
    // let tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter_config(WithExporterConfiguration::default())
    //     .install_batch(opentelemetry::runtime::Tokio)?;

    // let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    /*

    let subscriber1 = Registry::default()
        .with(tracing_subscriber::EnvFilter::new(log_level.clone())) // drop debug/trace, keep info+
        .with(kafka_layer)
        .with(otel_layer)
        .with(fmt::layer().compact().with_target(true));
    subscriber1.init();
     */
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let default_port = 6101;
    let port = env::var(format!("{}_PORT", service_name.clone()))
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| default_port);
    let log_dir = env::var("RUST_LOG_DIRECTORY").unwrap_or_else(|_| "./logs".to_string());
    let log_dir = format!("{}/{}", log_dir, service_name.to_lowercase());
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    let log_file_name = format!("{}_{}.log", service_name, port);
    let log_appender = rolling::daily(log_dir, log_file_name.to_lowercase());
    /*
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_writer(log_appender)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    */
    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_appender)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_ansi(false);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level.clone()))
        .with(log_layer)
        .with(tracing_subscriber::fmt::layer().with_thread_names(true)) // Log to stdout
        .with(kafka_layer)
        .with(otel_layer)
        .init();
    info!("Tracing subscriber initialized");

    Ok(())
}
