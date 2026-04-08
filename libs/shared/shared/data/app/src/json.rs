use axum::{
    extract::{FromRequest, Json, Request},
    response::{IntoResponse, Response},
    RequestExt,
};
use tracing::debug;
use validator::Validate;

use shared_shared_data_error::app::AppError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct ResponseJson<T>(pub T);

impl<T> IntoResponse for ResponseJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;
    use axum::response::Response as AxumResponse;
    use axum::body::to_bytes;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Clone)]
    struct TestPayload {
        #[validate(length(min = 1))]
        name: String,
    }

    #[tokio::test]
    async fn test_response_json_into_response() {
        let payload = TestPayload { name: "Alice".to_string() };
        let response_json = ResponseJson(payload.clone());
        let response: AxumResponse = response_json.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let deserialized: TestPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(deserialized, payload);
    }

    #[tokio::test]
    async fn test_valid_json_success() {
        let payload = TestPayload { name: "Bob".to_string() };
        let json = serde_json::to_string(&payload).unwrap();
        let req = HttpRequest::builder()
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(json))
            .unwrap();
        let result = ValidJson::<TestPayload>::from_request(req, &()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, payload);
    }

    #[tokio::test]
    async fn test_valid_json_validation_error() {
        let payload = TestPayload { name: "".to_string() }; // invalid: name too short
        let json = serde_json::to_string(&payload).unwrap();
        let req = HttpRequest::builder()
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(json))
            .unwrap();
        let result = ValidJson::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn test_valid_json_json_rejection() {
        // Invalid JSON
        let req = HttpRequest::builder()
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from("not a json"))
            .unwrap();
        let result = ValidJson::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::JsonRejection)));
    }
}

#[derive(Clone)]
pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    T: Validate + 'static,
{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = req.extract::<Json<T>, _>().await.map_err(|_err| {
            debug!(
                "Failed to extract Json. Response {:?}",
                _err.into_response()
            );
            AppError::JsonRejection
        })?;

        payload
            .validate()
            .map_err(|err| AppError::Validation(err))?;

        Ok(Self(payload))
    }
}
