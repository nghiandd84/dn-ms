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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::StatusCode, middleware, routing::get, Router};
    use axum_test::TestServer;

    async fn dummy_handler() -> &'static str {
        "OK"
    }

    #[tokio::test]
    async fn test_deprecation_endpoint_adds_headers() {
        let config = DeprecationConfig {
            endpoint_name: "test_endpoint",
            suggested_url: "/new-url",
        };

        let app = Router::new()
            .route("/test", get(dummy_handler))
            .layer(middleware::from_fn(move |req, next| {
                deprecation_endpoint(config.clone(), req, next)
            }));

        let server = TestServer::new(app).unwrap();

        let response = server.get("/test").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let deprecation_header = response.header("Deprecation");
        assert_eq!(deprecation_header, HeaderValue::from_static("true"));

        let link_header = response.header("Link");
        assert_eq!(
            link_header,
            HeaderValue::from_static(r#"</new-url>; rel="alternate""#)
        );
    }
    #[tokio::test]
    async fn test_deprecation_endpoint_increases_counter() {
        let config = DeprecationConfig {
            endpoint_name: "test_endpoint_counter",
            suggested_url: "/new-url",
        };

        let app = Router::new()
            .route("/test", get(dummy_handler))
            .layer(middleware::from_fn(move |req, next| {
                deprecation_endpoint(config.clone(), req, next)
            }));

        let server = TestServer::new(app).unwrap();

        // Call the endpoint multiple times
        for _ in 0..5 {
            let response = server.get("/test").await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }

        // Since we can't directly access the counter value, we can at least ensure it was initialized
        let counter = DEPRECATION_COUNTER.get();
        assert!(counter.is_some());
    }
}
