use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use features_inventory_model::{
    reservation::{
        ReservationData, ReservationDataFilterParams, ReservationForCreateRequest,
        ReservationForUpdateRequest,
    },
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

use features_inventory_service::ReservationService;

const TAG: &str = "reservation";

#[utoipa::path(
    post,
    path = "/reservations",
    tag = TAG,
    request_body = ReservationForCreateRequest,
    responses(
        (status = 201, description = "Reservation created successfully", body = OkUuidResponse),
    )
)]
async fn create_reservation(
    ValidJson(req): ValidJson<ReservationForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let reservation_id = ReservationService::create_reservation(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(reservation_id),
    }))
}

#[utoipa::path(
    get,
    path = "/reservations/{reservation_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Reservation retrieved successfully", body = ReservationData),
    )
)]
async fn get_reservation(
    Path(reservation_id): Path<Uuid>,
) -> Result<ResponseJson<ReservationData>> {
    let reservation = ReservationService::get_reservation_by_id(reservation_id).await?;

    debug!(
        "Recorded counter measurement for reservation retrieval: {:?}",
        reservation_id
    );
    Ok(ResponseJson(reservation))
}

#[utoipa::path(
    get,
    path = "/reservations",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered reservations", body = QueryResultResponse<ReservationData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_reservations(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<ReservationDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ReservationData>>> {
    debug!("Filtering reservations with params: {:?}", filter_params);
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = ReservationService::get_reservations(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/reservations/{reservation_id}",
    tag = TAG,
    request_body = ReservationForUpdateRequest,
    responses(
        (status = 200, description = "Reservation updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_reservation(
    Path(reservation_id): Path<Uuid>,
    ValidJson(req): ValidJson<ReservationForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ReservationService::update_reservation(reservation_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(reservation_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/reservations/{reservation_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Reservation deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_reservation(Path(reservation_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    ReservationService::delete_reservation(reservation_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(reservation_id),
    }))
}

pub fn routes(app_state: &AppState<InventoryAppState, InventoryCacheState>) -> Router {
    Router::new()
        .route("/reservations", post(create_reservation))
        .route("/reservations", get(filter_reservations))
        .route("/reservations/{reservation_id}", get(get_reservation))
        .route("/reservations/{reservation_id}", patch(update_reservation))
        .route("/reservations/{reservation_id}", delete(delete_reservation))
        .with_state(app_state.clone())
}
