use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::json::ResponseJson;
use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_url_shortener_model::{
    state::{UrlShortenerAppState, UrlShortenerCacheState},
    url_click::UrlClickData,
};
use features_url_shortener_service::url_click::UrlClickService;

use crate::permission::CanViewAnalytics;

const TAG: &str = "url-click";

#[utoipa::path(
    get,
    path = "/urls/{id}/clicks",
    tag = TAG,
    params(Pagination, Order),
    responses(
        (status = 200, description = "List of clicks for a URL", body = QueryResultResponse<UrlClickData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_url_clicks(
    _auth: Auth<CanViewAnalytics>,
    Path(id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<UrlClickData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = UrlClickService::get_clicks(&id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<UrlShortenerAppState, UrlShortenerCacheState>) -> Router {
    Router::new()
        .route("/urls/{id}/clicks", get(get_url_clicks))
        .with_state(app_state.clone())
}
