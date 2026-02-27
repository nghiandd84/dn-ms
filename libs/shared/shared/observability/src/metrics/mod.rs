use opentelemetry::global;
use opentelemetry_sdk::{metrics::SdkMeterProvider, Resource};

pub fn init_metrics_provider(service_name: String) -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporterBuilder::new()
        // .with_tonic()
        .with_http()
        .build()
        .expect("Failed to create metric exporter");
    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .build();
    global::set_meter_provider(provider.clone());
    provider
}
