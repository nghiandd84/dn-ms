use axum::{
    extract::Request, http::HeaderValue, middleware::Next, response::IntoResponse, Extension,
};
use opentelemetry::{global, metrics::Counter, KeyValue};
use std::sync::{Arc, OnceLock};
use tracing::debug;

#[derive(Clone)]
pub struct DeprecationConfig {
    pub endpoint_name: &'static str, // e.g., "v1_get_users"
    pub suggested_url: &'static str,
}

pub static DEPRECATION_COUNTER: OnceLock<Arc<Counter<u64>>> = OnceLock::new();

pub async fn deprecation_tracking_middleware(
    config: Option<Extension<DeprecationConfig>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    debug!(
        "Deprecation middleware invoked for path: {}",
        req.uri().path()
    );

    // TODO fix bug config is None when using from_fn_with_state, but works with Extension. Need to investigate further.

    // 1. Record the metric if the extension is present
    if let Some(Extension(cfg)) = &config {
        debug!(
            "Incrementing deprecation counter for endpoint: {}",
            cfg.endpoint_name
        );
        let counter = DEPRECATION_COUNTER
            .get_or_init(|| {
                let meter = global::meter_provider().meter_with_scope(
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
        counter.add(1, &[KeyValue::new("endpoint", cfg.endpoint_name)]);
    }

    // 2. Run the actual handler
    let mut response = next.run(req).await;

    // 3. Add the deprecation headers to the response
    if let Some(Extension(cfg)) = config {
        debug!(
            "Adding deprecation headers for endpoint: {}, suggested URL: {}",
            cfg.endpoint_name, cfg.suggested_url
        );
        response
            .headers_mut()
            .insert("Deprecation", HeaderValue::from_static("true"));
        response.headers_mut().insert(
            "Link",
            HeaderValue::from_str(&format!(r#"<{}>; rel="alternate""#, cfg.suggested_url)).unwrap(),
        );
    }

    response
}
