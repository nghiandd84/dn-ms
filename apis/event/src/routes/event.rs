use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_event_model::{
    state::{EventAppState, EventCacheState},
    EventData, EventDataFilterParams, EventForCreateRequest, EventForUpdateRequest,
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_event_service::EventService;
use features_event_stream::PRODUCER_KEY;

const TAG: &str = "event";

#[utoipa::path(
    post,
    path = "/events",
    tag = TAG,
    request_body = EventForCreateRequest,
    responses(
        (status = 201, description = "Event created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_event(
    state: State<AppState<EventAppState, EventCacheState>>,
    Json(req): Json<EventForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let event_id = EventService::create_event(&state.conn, req, &producer).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(event_id),
    }))
}

#[utoipa::path(
    get,
    path = "/events/{event_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Event retrieved successfully", body = EventData),
    )
)]
async fn get_event(
    state: State<AppState<EventAppState, EventCacheState>>,
    Path(event_id): Path<Uuid>,
) -> Result<ResponseJson<EventData>> {
    let event = EventService::get_event_by_id(&state.conn, event_id).await?;
    Ok(ResponseJson(event))
}

#[utoipa::path(
    get,
    path = "/events",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered events", body = QueryResultResponse<EventData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_events(
    state: State<AppState<EventAppState, EventCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<EventDataFilterParams>,
) -> Result<ResponseJson<QueryResult<EventData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = EventService::get_events(&state.conn, &filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/events/{event_id}",
    tag = TAG,
    request_body = EventForUpdateRequest,
    responses(
        (status = 200, description = "Event updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_event(
    state: State<AppState<EventAppState, EventCacheState>>,
    Path(event_id): Path<Uuid>,
    Json(req): Json<EventForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
        let producer = state
            .get_producer(PRODUCER_KEY.to_string())
            .expect("Producer not found");
    EventService::update_event(&state.conn, event_id, req, &producer).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(event_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/events/{event_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Event deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_event(
    state: State<AppState<EventAppState, EventCacheState>>,
    Path(event_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    EventService::delete_event(&state.conn, event_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(event_id),
    }))
}

pub fn routes(app_state: &AppState<EventAppState, EventCacheState>) -> Router {
    Router::new()
        .route("/events", post(create_event))
        .route("/events", get(filter_events))
        .route("/events/{event_id}", get(get_event))
        .route("/events/{event_id}", patch(update_event))
        .route("/events/{event_id}", delete(delete_event))
        .with_state(app_state.clone())
}
