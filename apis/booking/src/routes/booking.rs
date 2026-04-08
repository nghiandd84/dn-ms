use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_booking_model::{
    booking::{
        BookingData, BookingDataFilterParams, BookingForCreateRequest, BookingForUpdateRequest,
    },
    state::{BookingAppState, BookingCacheState},
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

use features_booking_service::BookingService;

const TAG: &str = "booking";

#[utoipa::path(
    post,
    path = "/bookings",
    tag = TAG,
    request_body = BookingForCreateRequest,
    responses(
        (status = 201, description = "Booking created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_booking(
    ValidJson(req): ValidJson<BookingForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let booking_id = BookingService::create_booking(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_id),
    }))
}

#[utoipa::path(
    get,
    path = "/bookings/{booking_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking retrieved successfully", body = BookingData),
    )
)]
async fn get_booking(Path(booking_id): Path<Uuid>) -> Result<ResponseJson<BookingData>> {
    let booking = BookingService::get_booking_by_id(booking_id).await?;
    Ok(ResponseJson(booking))
}

#[utoipa::path(
    get,
    path = "/bookings",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered bookings", body = QueryResultResponse<BookingData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_bookings(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<BookingDataFilterParams>,
) -> Result<ResponseJson<QueryResult<BookingData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = BookingService::get_bookings(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/bookings/{booking_id}",
    tag = TAG,
    request_body = BookingForUpdateRequest,
    responses(
        (status = 200, description = "Booking updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_booking(
    Path(booking_id): Path<Uuid>,
    ValidJson(req): ValidJson<BookingForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    BookingService::update_booking(booking_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/bookings/{booking_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_booking(Path(booking_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    BookingService::delete_booking(booking_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_id),
    }))
}

pub fn routes(app_state: &AppState<BookingAppState, BookingCacheState>) -> Router {
    Router::new()
        .route("/bookings", post(create_booking))
        .route("/bookings", get(filter_bookings))
        .route("/bookings/{booking_id}", get(get_booking))
        .route("/bookings/{booking_id}", patch(update_booking))
        .route("/bookings/{booking_id}", delete(delete_booking))
        .with_state(app_state.clone())
}
