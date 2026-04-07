use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::Next,
};
use std::time::Duration;
use tracing::{debug, error};

use features_lookup_model::state::{LookupAppState, LookupCacheState};
use shared_shared_app::state::AppState;

const CACHE_KEY_PREFIX: &str = "lookup_items";
const CACHE_TTL_SECONDS: u64 = 60;

pub async fn cache_lookup_items_middleware(
    State(app_state): State<AppState<LookupAppState, LookupCacheState>>,
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = req.uri().path().to_string();
    let query_string = req.uri().query().unwrap_or("");
    let cache_key = format!("{}:{}-{}", CACHE_KEY_PREFIX, path, query_string);
    let cached_response = app_state.cache.get(&cache_key);

    match cached_response {
        Ok(Some(cached_body)) => {
            debug!("Lookup items cache hit: {}", cache_key);
            match cached_body {
                LookupCacheState::LookupItems { datas } => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/json")
                        .header("x-cache", "HIT")
                        .header(
                            "x-cache-expires-in",
                            format!("{} seconds", CACHE_TTL_SECONDS),
                        )
                        .body(Body::from(datas.clone()))
                        .unwrap();
                }
                _ => {
                    debug!(
                        "Cache entry found but in unexpected format for key: {}",
                        cache_key
                    );
                }
            }
        }
        Ok(None) => debug!("Lookup items cache miss: {}", cache_key),
        Err(err) => error!("Lookup items cache get failed: {:?}", err),
    }

    let response = next.run(req).await;
    let status = response.status();
    let (parts, body) = response.into_parts();

    let body_bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Failed to read response body for cache: {:?}", err);
            return Response::from_parts(parts, Body::empty());
        }
    };

    if status == StatusCode::OK {
        if let Ok(body_string) = std::str::from_utf8(&body_bytes).map(|s| s.to_string()) {
            let lookup_item_cache_state = LookupCacheState::LookupItems { datas: body_string };
            if let Err(err) = app_state.cache.insert(
                cache_key,
                lookup_item_cache_state,
                Some(Duration::from_secs(CACHE_TTL_SECONDS)),
            ) {
                error!("Failed to insert lookup items cache: {:?}", err);
            } else {
                debug!("Cached lookup items response for {}", path);
            }
        }
    }

    let mut response = Response::from_parts(parts, Body::from(body_bytes));
    response.headers_mut().insert(
        "x-cache-expires-in",
        format!("{} seconds", CACHE_TTL_SECONDS)
            .parse()
            .unwrap_or_else(|_| format!("{} seconds", CACHE_TTL_SECONDS).parse().unwrap()),
    );
    response
}
