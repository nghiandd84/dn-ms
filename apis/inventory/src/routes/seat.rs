use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_inventory_model::{
    seat::{SeatData, SeatDataFilterParams, SeatForCreateRequest, SeatForUpdateRequest},
    state::{InventoryAppState, InventoryCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_inventory_service::SeatService;

const TAG: &str = "seat";

#[utoipa::path(
    post,
    path = "/seats",
    tag = TAG,
    request_body = SeatForCreateRequest,
    responses(
        (status = 201, description = "Seat created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_seat(
    state: State<AppState<InventoryAppState, InventoryCacheState>>,
    ValidJson(req): ValidJson<SeatForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let seat_id = SeatService::create_seat(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(seat_id),
    }))
}

#[utoipa::path(
    get,
    path = "/seats/{seat_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Seat retrieved successfully", body = SeatData),
    )
)]
async fn get_seat(
    state: State<AppState<InventoryAppState, InventoryCacheState>>,
    Path(seat_id): Path<Uuid>,
) -> Result<ResponseJson<SeatData>> {
    let seat = SeatService::get_seat_by_id(&state.conn, seat_id).await?;
    Ok(ResponseJson(seat))
}

#[utoipa::path(
    get,
    path = "/seats",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered seats", body = QueryResultResponse<SeatData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_seats(
    state: State<AppState<InventoryAppState, InventoryCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<SeatDataFilterParams>,
) -> Result<ResponseJson<QueryResult<SeatData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = SeatService::get_seats(&state.conn, &filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/seats/{seat_id}",
    tag = TAG,
    request_body = SeatForUpdateRequest,
    responses(
        (status = 200, description = "Seat updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_seat(
    state: State<AppState<InventoryAppState, InventoryCacheState>>,
    Path(seat_id): Path<Uuid>,
    ValidJson(req): ValidJson<SeatForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    SeatService::update_seat(seat_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(seat_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/seats/{seat_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Seat deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_seat(
    state: State<AppState<InventoryAppState, InventoryCacheState>>,
    Path(seat_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    SeatService::delete_seat(seat_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(seat_id),
    }))
}

pub fn routes(app_state: &AppState<InventoryAppState, InventoryCacheState>) -> Router {
    Router::new()
        .route("/seats", post(create_seat))
        .route("/seats", get(filter_seats))
        .route("/seats/{seat_id}", get(get_seat))
        .route("/seats/{seat_id}", patch(update_seat))
        .route("/seats/{seat_id}", delete(delete_seat))
        .with_state(app_state.clone())
}
