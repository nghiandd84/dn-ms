use std::any::Any;

use axum::{
    extract::{FromRequest, Json, Request},
    response::{IntoResponse, Response},
    RequestExt,
};
use tracing::debug;
use validator::Validate;

use crate::error::AppError;

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

// TODO Need to implement ValidateArgs
/*
use validator::ValidateArgs;
impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    T: for<'a> ValidateArgs<'a, Args = ValidContext<T>> + Clone + 'static,

{
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|_err| AppError::JsonRejection)?;

        let valid_context = ValidContext {
            data: payload.clone(),
        };
        payload
            .validate_with_args(valid_context)
            .map_err(|err| AppError::Validation(err))?;

        Ok(Self(payload))
    }
}

#[derive(Clone)]
pub struct ValidContext<T> {
    pub data: T,
}
*/
