use opentelemetry::KeyValue;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};

pub fn init_otel_traces(service_name: String) -> SdkTracerProvider {
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
