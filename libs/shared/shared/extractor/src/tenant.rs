use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Debug, Clone)]
pub struct TenantId(pub String);

impl<S: Send + Sync> FromRequestParts<S> for TenantId {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let tenant_id = parts
            .headers
            .get("baggage")
            .and_then(|v| v.to_str().ok())
            .and_then(|baggage| {
                baggage
                    .split(',')
                    .filter_map(|kv| {
                        let mut pieces = kv.splitn(2, '=');
                        let key = pieces.next()?.trim();
                        let val = pieces.next()?.trim();
                        if key == "tenant_id" {
                            Some(val.to_string())
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .unwrap_or_default();
        Ok(TenantId(tenant_id))
    }
}
