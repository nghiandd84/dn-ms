use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_booking_model::{
    guest_booking::{
        GuestBookingAdminForCreateRequest, GuestBookingConfirmRequest, GuestBookingData,
        GuestBookingDataFilterParams, GuestBookingForCreateRequest, GuestBookingForUpdateRequest,
    },
    guest_booking_history::GuestBookingHistoryData,
    state::{BookingAppState, BookingCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::{Auth, PublicAccess};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
    query_params::QueryParams,
};

use crate::permission::{
    CanCreateBooking, CanCreateGuestBooking, CanDeleteGuestBooking, CanReadGuestBooking,
    CanUpdateGuestBooking,
};
use features_booking_service::GuestBookingService;
use features_booking_stream::PRODUCER_KEY;

const TAG: &str = "guest_booking";

// ============================ PUBLIC (no auth) ============================

#[utoipa::path(
    post,
    path = "/public/guest-bookings",
    tag = TAG,
    request_body = GuestBookingForCreateRequest,
    responses(
        (status = 201, description = "Guest booking created; confirm token is emailed out-of-band", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_guest_booking(
    _public: PublicAccess,
    state: State<AppState<BookingAppState, BookingCacheState>>,
    ValidJson(req): ValidJson<GuestBookingForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let guest_booking_id = GuestBookingService::create_guest_booking(req, &producer).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(guest_booking_id),
    }))
}

#[utoipa::path(
    get,
    path = "/public/guest-bookings/{guest_booking_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Guest booking retrieved successfully", body = GuestBookingData),
    )
)]
async fn get_guest_booking(
    _public: PublicAccess,
    Path(guest_booking_id): Path<Uuid>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<GuestBookingData>> {
    let guest_booking =
        GuestBookingService::get_guest_booking_by_id(guest_booking_id, &query_params).await?;
    Ok(ResponseJson(guest_booking))
}

#[utoipa::path(
    post,
    path = "/public/guest-bookings/{guest_booking_id}/confirm",
    tag = TAG,
    request_body = GuestBookingConfirmRequest,
    responses(
        (status = 200, description = "Guest booking confirmed successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn confirm_guest_booking(
    _public: PublicAccess,
    Path(guest_booking_id): Path<Uuid>,
    ValidJson(req): ValidJson<GuestBookingConfirmRequest>,
) -> Result<ResponseJson<OkUuid>> {
    GuestBookingService::confirm_guest_booking(guest_booking_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(guest_booking_id),
    }))
}

// ======================= AUTHENTICATED (OAuth) =======================

#[utoipa::path(
    post,
    path = "/guest-bookings/{guest_booking_id}/promote",
    tag = TAG,
    responses(
        (status = 200, description = "Guest booking promoted to a real booking", body = OkUuidResponse),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn promote_guest_booking(
    auth: Auth<CanCreateBooking>,
    Path(guest_booking_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    let booking_id =
        GuestBookingService::promote_guest_booking(guest_booking_id, auth.user_id()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_id),
    }))
}

// ===================== ADMIN (authenticated management) =====================

#[utoipa::path(
    post,
    path = "/guest-bookings",
    tag = TAG,
    request_body = GuestBookingAdminForCreateRequest,
    responses(
        (status = 201, description = "Guest booking created by an administrator", body = OkUuidResponse),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_guest_booking_admin(
    auth: Auth<CanCreateGuestBooking>,
    ValidJson(req): ValidJson<GuestBookingAdminForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let guest_booking_id =
        GuestBookingService::create_guest_booking_admin(req, auth.user_id()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(guest_booking_id),
    }))
}

#[utoipa::path(
    get,
    path = "/guest-bookings",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered guest bookings", body = QueryResultResponse<GuestBookingData>),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_guest_bookings(
    _auth: Auth<CanReadGuestBooking>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<GuestBookingDataFilterParams>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<GuestBookingData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        GuestBookingService::get_guest_bookings(&filters, &pagination, &order, &query_params)
            .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/guest-bookings/{guest_booking_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Guest booking retrieved by an administrator", body = GuestBookingData),
    ),
    security(("jwt" = []))
)]
async fn get_guest_booking_admin(
    _auth: Auth<CanReadGuestBooking>,
    Path(guest_booking_id): Path<Uuid>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<GuestBookingData>> {
    let guest_booking =
        GuestBookingService::get_guest_booking_by_id(guest_booking_id, &query_params).await?;
    Ok(ResponseJson(guest_booking))
}

#[utoipa::path(
    patch,
    path = "/guest-bookings/{guest_booking_id}",
    tag = TAG,
    request_body = GuestBookingForUpdateRequest,
    responses(
        (status = 200, description = "Guest booking updated by an administrator", body = OkUuidResponse),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_guest_booking_admin(
    auth: Auth<CanUpdateGuestBooking>,
    Path(guest_booking_id): Path<Uuid>,
    ValidJson(req): ValidJson<GuestBookingForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    GuestBookingService::update_guest_booking_admin(guest_booking_id, req, auth.user_id()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(guest_booking_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/guest-bookings/{guest_booking_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Guest booking cancelled by an administrator", body = OkUuidResponse),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_guest_booking_admin(
    auth: Auth<CanDeleteGuestBooking>,
    Path(guest_booking_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    GuestBookingService::delete_guest_booking_admin(guest_booking_id, auth.user_id()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(guest_booking_id),
    }))
}

#[utoipa::path(
    get,
    path = "/guest-bookings/{guest_booking_id}/history",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Guest booking lifecycle history", body = QueryResultResponse<GuestBookingHistoryData>),
    ),
    security(("jwt" = []))
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_guest_booking_history(
    _auth: Auth<CanReadGuestBooking>,
    Path(guest_booking_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<GuestBookingHistoryData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result =
        GuestBookingService::get_guest_booking_history(guest_booking_id, &pagination, &order)
            .await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<BookingAppState, BookingCacheState>) -> Router {
    Router::new()
        // public guest flow (no auth; /public prefix is exempt from baggage)
        .route("/public/guest-bookings", post(create_guest_booking))
        .route(
            "/public/guest-bookings/{guest_booking_id}",
            get(get_guest_booking),
        )
        .route(
            "/public/guest-bookings/{guest_booking_id}/confirm",
            post(confirm_guest_booking),
        )
        // authenticated promotion (runs after OAuth + payment)
        .route(
            "/guest-bookings/{guest_booking_id}/promote",
            post(promote_guest_booking),
        )
        // admin management (authenticated CRUD + search)
        .route("/guest-bookings", post(create_guest_booking_admin))
        .route("/guest-bookings", get(filter_guest_bookings))
        .route(
            "/guest-bookings/{guest_booking_id}",
            get(get_guest_booking_admin),
        )
        .route(
            "/guest-bookings/{guest_booking_id}",
            patch(update_guest_booking_admin),
        )
        .route(
            "/guest-bookings/{guest_booking_id}",
            delete(delete_guest_booking_admin),
        )
        .route(
            "/guest-bookings/{guest_booking_id}/history",
            get(get_guest_booking_history),
        )
        .with_state(app_state.clone())
}
