use std::env;

use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn init_otel_logger_provider(service_name: String) -> SdkLoggerProvider {
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

pub fn init_rolling_file_appender(service_name: String) -> RollingFileAppender {
    let default_port = 6101;
    let port = env::var(format!("{}_PORT", service_name.clone()))
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or(default_port);
    let log_dir = env::var("RUST_LOG_DIRECTORY").unwrap_or_else(|_| "./logs".to_string());
    let log_dir = format!("{}/{}", log_dir, service_name.to_lowercase());
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    let log_file_name = format!("{}_{}.log", service_name, port).to_lowercase();

    RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix(log_file_name)
        .build(log_dir)
        .expect("Failed to create log appender")
}
