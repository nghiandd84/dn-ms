use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use opentelemetry::{global, metrics::Counter, KeyValue};
use std::sync::{Arc, OnceLock};
use tracing::debug;

static DEPRECATION_COUNTER: OnceLock<Arc<Counter<u64>>> = OnceLock::new();

#[derive(Clone)]
pub struct DeprecationConfig {
    pub endpoint_name: &'static str, // e.g., "v1_get_users"
    pub suggested_url: &'static str,
}

pub async fn deprecation_endpoint(config: DeprecationConfig, req: Request, next: Next) -> Response {
    debug!(
        "Deprecation config found: endpoint_name={}, suggested_url={}",
        config.endpoint_name, config.suggested_url
    );

    let counter = DEPRECATION_COUNTER
        .get_or_init(|| {
            debug!("Initializing deprecation counter");
            let _meter = global::meter_provider().meter_with_scope(
                opentelemetry::InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
                    .with_version(env!("CARGO_PKG_VERSION"))
                    .build(),
            );
            let provider = global::meter("deprecation_middleware");
            Arc::new(
                provider
                    .u64_counter("api_deprecation_calls_total")
                    .with_description("Counts hits to deprecated endpoints")
                    .build(),
            )
        })
        .clone();
    counter.add(1, &[KeyValue::new("endpoint", config.endpoint_name)]);

    // Continue the request chain
    let mut response = next.run(req).await;
    response
        .headers_mut()
        .insert("Deprecation", HeaderValue::from_static("true"));
    response.headers_mut().insert(
        "Link",
        HeaderValue::from_str(&format!(r#"<{}>; rel="alternate""#, config.suggested_url)).unwrap(),
    );
    response
}
