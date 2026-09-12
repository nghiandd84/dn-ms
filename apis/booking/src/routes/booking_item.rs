use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_booking_model::{
    booking_item::{
        BookingItemData, BookingItemDataFilterParams, BookingItemForCreateRequest,
        BookingItemForUpdateRequest,
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

use crate::permission::{CanCreateItem, CanDeleteItem, CanReadItem, CanUpdateItem};
use features_booking_service::BookingItemService;

const TAG: &str = "booking_item";

#[utoipa::path(
    post,
    path = "/booking-items",
    tag = TAG,
    request_body = BookingItemForCreateRequest,
    responses(
        (status = 201, description = "Booking item created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_booking_item(
    _auth: Auth<CanCreateItem>,
    ValidJson(req): ValidJson<BookingItemForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let booking_item_id = BookingItemService::create_booking_item(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_item_id),
    }))
}

#[utoipa::path(
    get,
    path = "/booking-items/{booking_item_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking item retrieved successfully", body = BookingItemData),
    )
)]
async fn get_booking_item(
    _auth: Auth<CanReadItem>,
    Path(booking_item_id): Path<Uuid>,
) -> Result<ResponseJson<BookingItemData>> {
    let booking_item = BookingItemService::get_booking_item_by_id(booking_item_id).await?;
    Ok(ResponseJson(booking_item))
}

#[utoipa::path(
    get,
    path = "/booking-items",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered booking items", body = QueryResultResponse<BookingItemData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_booking_items(
    _auth: Auth<CanReadItem>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<BookingItemDataFilterParams>,
) -> Result<ResponseJson<QueryResult<BookingItemData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = BookingItemService::get_booking_items(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/booking-items/{booking_item_id}",
    tag = TAG,
    request_body = BookingItemForUpdateRequest,
    responses(
        (status = 200, description = "Booking item updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_booking_item(
    _auth: Auth<CanUpdateItem>,
    Path(booking_item_id): Path<Uuid>,
    ValidJson(req): ValidJson<BookingItemForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    BookingItemService::update_booking_item(booking_item_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_item_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/booking-items/{booking_item_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Booking item deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_booking_item(
    _auth: Auth<CanDeleteItem>,
    Path(booking_item_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    BookingItemService::delete_booking_item(booking_item_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(booking_item_id),
    }))
}

pub fn routes(app_state: &AppState<BookingAppState, BookingCacheState>) -> Router {
    Router::new()
        .route("/booking-items", post(create_booking_item))
        .route("/booking-items", get(filter_booking_items))
        .route("/booking-items/{booking_item_id}", get(get_booking_item))
        .route(
            "/booking-items/{booking_item_id}",
            patch(update_booking_item),
        )
        .route(
            "/booking-items/{booking_item_id}",
            delete(delete_booking_item),
        )
        .with_state(app_state.clone())
}
