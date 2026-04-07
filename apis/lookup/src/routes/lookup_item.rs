use axum::{
    extract::{Path, Query},
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
    Router,
};
use shared_shared_data_error::app::AppError;
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use features_lookup_model::{
    lookup_item::{
        LookupItemData, LookupItemDataFilterParams, LookupItemForCreateRequest,
        LookupItemForUpdateRequest,
    },
    state::{LookupAppState, LookupCacheState},
};
use features_lookup_service::{lookup_item::LookupItemService, lookup_type::LookupTypeService};

use shared_shared_app::state::AppState;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use crate::middleware::cache_lookup_items_middleware;

const TAG: &str = "lookup-item";

#[utoipa::path(
    get,
    path = "/lookup-types/{type_code}/items",
    tag = TAG,
    params(Pagination),
    responses(
        (status = 200, description = "List of lookup items", body = QueryResultResponse<LookupItemData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_lookup_items(
    Path(type_code): Path<String>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<LookupItemDataFilterParams>,
) -> Result<ResponseJson<QueryResult<LookupItemData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        LookupItemService::get_lookup_items_by_type_code(&type_code, &filters, &pagination, &order)
            .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/lookup-types/{type_code}/items/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Lookup item", body = LookupItemData),
    )
)]
pub async fn get_lookup_item(
    Path((_type_code, id)): Path<(String, Uuid)>,
) -> Result<ResponseJson<LookupItemData>> {
    let result = LookupItemService::get_lookup_item_by_id(id).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/lookup-types/{type_code}/items",
    tag = TAG,
    request_body = LookupItemForCreateRequest,
    responses(
        (status = 201, description = "Lookup item created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_lookup_item(
    Path(type_code): Path<String>,
    ValidJson(mut req): ValidJson<LookupItemForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let lookup_type = LookupTypeService::get_lookup_type_by_code(&type_code).await;
    let lookup_type_id = match lookup_type {
        Ok(lookup_type) => lookup_type.id.unwrap(),
        Err(e) => {
            debug!("Error fetching lookup type by code {}: {:?}", type_code, e);
            return Err(AppError::Internal(
                "Failed to fetch lookup type".to_string(),
            ));
        }
    };
    req.lookup_type_id = Some(lookup_type_id);
    let id = LookupItemService::create_lookup_item(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    patch,
    path = "/lookup-types/{type_code}/items/{id}",
    tag = TAG,
    request_body = LookupItemForUpdateRequest,
    responses(
        (status = 200, description = "Lookup item updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_lookup_item(
    Path((_type_code, id)): Path<(String, Uuid)>,
    ValidJson(req): ValidJson<LookupItemForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    LookupItemService::update_lookup_item(id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/lookup-types/{type_code}/items/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Lookup item deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_lookup_item(
    Path((_type_code, id)): Path<(String, Uuid)>,
) -> Result<ResponseJson<OkUuid>> {
    LookupItemService::delete_lookup_item(id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<LookupAppState, LookupCacheState>) -> Router {
    Router::new()
        .route(
            "/lookup-types/{type_code}/items",
            get(get_lookup_items).layer(from_fn_with_state(app_state.clone(), cache_lookup_items_middleware)),
        )
        .route("/lookup-types/{type_code}/items", post(create_lookup_item))
        .route("/lookup-types/{type_code}/items/{id}", get(get_lookup_item))
        .route(
            "/lookup-types/{type_code}/items/{id}",
            patch(update_lookup_item),
        )
        .route(
            "/lookup-types/{type_code}/items/{id}",
            delete(delete_lookup_item),
        )
        .with_state(app_state.clone())
}
