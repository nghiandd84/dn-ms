use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_booking_model::{
    guest_booking::{
        GuestBookingConfirmRequest, GuestBookingData, GuestBookingForCreateRequest,
    },
    state::{BookingAppState, BookingCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::{Auth, PublicAccess};
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::query_params::QueryParams;

use crate::permission::CanCreateBooking;
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
        .with_state(app_state.clone())
}
