use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};

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
