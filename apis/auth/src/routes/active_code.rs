use axum::{routing::post, Router};
use serde::Deserialize;
use tracing::debug;
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

use features_auth_model::state::{AuthAppState, AuthCacheState};
use features_auth_service::ActiveCodeService;
use shared_shared_app::state::AppState;
use shared_shared_auth::permission::PublicAccess;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct MarkAsSentRequest {
    pub user_id: Uuid,
    pub code: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct MarkAsSentResponse {
    pub marked: bool,
}

#[utoipa::path(
    post,
    request_body = MarkAsSentRequest,
    path = "/internal/active-codes/mark-sent",
    tag = "internal",
    summary = "Mark active code as sent",
    description = "Atomically marks an active code as sent. Returns marked=false if already sent.",
    responses(
        (status = 200, description = "Success", body = MarkAsSentResponse),
        (status = 400, description = "Bad request"),
    )
)]
pub async fn mark_as_sent(
    _public: PublicAccess,
    ValidJson(request): ValidJson<MarkAsSentRequest>,
) -> Result<ResponseJson<MarkAsSentResponse>> {
    debug!(
        "Mark active code as sent: user_id={}, code={}",
        request.user_id, request.code
    );
    let marked = ActiveCodeService::mark_as_sent(request.user_id, request.code).await?;
    Ok(ResponseJson(MarkAsSentResponse { marked }))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/internal/active-codes/mark-sent", post(mark_as_sent))
        .with_state(app_state.clone())
}
