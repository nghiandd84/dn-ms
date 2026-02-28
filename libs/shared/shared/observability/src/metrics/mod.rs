pub mod axum_otel;

use opentelemetry::global;
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider, Temporality},
    Resource,
};

pub fn init_metrics_provider(service_name: String) -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporterBuilder::new()
        // .with_tonic()
        .with_http()
        .with_temporality(Temporality::default())
        .build()
        .expect("Failed to create metric exporter");
    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();
    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .build();
    global::set_meter_provider(provider.clone());
    provider
}
