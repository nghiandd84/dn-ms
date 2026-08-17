use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_event_model::{
    state::{EventAppState, EventCacheState},
    EventData, EventDataFilterParams,
};
use features_event_service::EventService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::PublicAccess;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::ResponseJson,
    result::Result,
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

const TAG: &str = "public-event";

#[utoipa::path(
    get,
    path = "/public/events",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Search public events", body = QueryResultResponse<EventData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn search_public_events(
    _public: PublicAccess,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<EventDataFilterParams>,
) -> Result<ResponseJson<QueryResult<EventData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = EventService::get_events(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/public/events/{event_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Get public event by ID", body = EventData),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_public_event(
    _public: PublicAccess,
    Path(event_id): Path<Uuid>,
) -> Result<ResponseJson<EventData>> {
    let event = EventService::get_event_by_id(event_id).await?;
    Ok(ResponseJson(event))
}

pub fn routes(app_state: &AppState<EventAppState, EventCacheState>) -> Router {
    Router::new()
        .route("/public/events", get(search_public_events))
        .route("/public/events/{event_id}", get(get_public_event))
        .with_state(app_state.clone())
}
