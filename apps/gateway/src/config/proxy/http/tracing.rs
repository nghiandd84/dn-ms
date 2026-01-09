use http::{HeaderName, HeaderValue};
use opentelemetry::propagation::{Extractor, Injector};
use pingora_http::RequestHeader;
use std::str::FromStr;
use tracing::debug;

// Helper for Extraction (Reading incoming headers)
pub struct PingoraHeaderExtractor<'a>(pub &'a RequestHeader);
impl<'a> Extractor for PingoraHeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        let data = self.0.headers.get(key).and_then(|v| v.to_str().ok());
        debug!(
            "Extracting header: {} with  value {}",
            key,
            data.unwrap_or("Unknown")
        );
        data
    }
    fn keys(&self) -> Vec<&str> {
        debug!("Getting all header keys");
        self.0.headers.iter().map(|(k, _)| k.as_str()).collect()
    }
}

// Helper for Injection (Writing outgoing headers)
pub struct PingoraHeaderInjector<'a>(pub &'a mut RequestHeader);
impl<'a> Injector for PingoraHeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        debug!("Injecting header: {}: {}", key, value);
        let header_value = HeaderValue::from_str(&value).unwrap();
        let header_name = HeaderName::from_str(key).unwrap();
        let _ = self.0.insert_header(header_name, header_value);
    }
}
