use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_booking_model::{
    booking_seat::{
        BookingSeatData, BookingSeatDataFilterParams, BookingSeatForCreateRequest,
        BookingSeatForUpdateRequest,
    },
    state::{BookingAppState, BookingCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use crate::permission::{CanCreateSeat, CanDeleteSeat, CanReadSeat, CanUpdateSeat};
use features_booking_service::BookingSeatService;

const TAG: &str = "booking_seat";

#[utoipa::path(
    post,
    path = "/booking-seats",
    tag = TAG,
    request_body = BookingSeatForCreateRequest,
    responses(
        (status = 201, description = "Booking seat created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_booking_seat(
    _auth: Auth<CanCreateSeat>,
    ValidJson(req): ValidJson<BookingSeatForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let booking_seat_id = BookingSeatService::create_booking_seat(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_seat_id),
    }))
}

#[utoipa::path(
    get,
    path = "/booking-seats/{booking_seat_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking seat retrieved successfully", body = BookingSeatData),
    )
)]
async fn get_booking_seat(
    _auth: Auth<CanReadSeat>,
    Path(booking_seat_id): Path<Uuid>,
) -> Result<ResponseJson<BookingSeatData>> {
    let booking_seat = BookingSeatService::get_booking_seat_by_id(booking_seat_id).await?;
    Ok(ResponseJson(booking_seat))
}

#[utoipa::path(
    get,
    path = "/booking-seats",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered booking seats", body = QueryResultResponse<BookingSeatData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_booking_seats(
    _auth: Auth<CanReadSeat>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<BookingSeatDataFilterParams>,
) -> Result<ResponseJson<QueryResult<BookingSeatData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = BookingSeatService::get_booking_seats(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/booking-seats/{booking_seat_id}",
    tag = TAG,
    request_body = BookingSeatForUpdateRequest,
    responses(
        (status = 200, description = "Booking seat updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_booking_seat(
    _auth: Auth<CanUpdateSeat>,
    Path(booking_seat_id): Path<Uuid>,
    ValidJson(req): ValidJson<BookingSeatForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    BookingSeatService::update_booking_seat(booking_seat_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_seat_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/booking-seats/{booking_seat_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking seat deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_booking_seat(
    _auth: Auth<CanDeleteSeat>,
    Path(booking_seat_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    BookingSeatService::delete_booking_seat(booking_seat_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_seat_id),
    }))
}

pub fn routes(app_state: &AppState<BookingAppState, BookingCacheState>) -> Router {
    Router::new()
        .route("/booking-seats", post(create_booking_seat))
        .route("/booking-seats", get(filter_booking_seats))
        .route("/booking-seats/{booking_seat_id}", get(get_booking_seat))
        .route(
            "/booking-seats/{booking_seat_id}",
            patch(update_booking_seat),
        )
        .route(
            "/booking-seats/{booking_seat_id}",
            delete(delete_booking_seat),
        )
        .with_state(app_state.clone())
}
