use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Router,
};
use features_bakery_service::cake_bakers::CakeBakerMutation;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkI32, OkI32Response, Result},
};

use features_bakery_entities::cakes_bakers::CakeBakerForCreateDto;
use features_bakery_model::cakes_bakers::CakeBakerForCreateRequest;

const TAG: &str = "cake-bakers";

#[utoipa::path(
    post,
    request_body = CakeBakerForCreateRequest,
    path = "/cake-bakers",
    tag = TAG,
    operation_id = "create-cake-baker",
    responses(
        (status = 200, description = "Cake is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState>,
    ValidJson(request): ValidJson<CakeBakerForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let dto: CakeBakerForCreateDto = request.into();
    let success = CakeBakerMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkI32 {
        ok: success,
        id: None,
    }))
}

#[utoipa::path(
    delete,
    path = "/cake-bakers/{cake_id}/{baker_id}",
    tag = TAG,
    operation_id = "delete-cake-baker-by-id",
    responses(
        (status = 200, description = "Cake Baker is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState>,
    Path((cake_id, baker_id)): Path<(i32, i32)>,
) -> Result<ResponseJson<OkI32>> {
    CakeBakerMutation::delete(&state.conn, cake_id, baker_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/cake-bakers", post(create))
        .route("/cake-bakers/{cake_id}/{baker_id}", delete(delete_by_id))
        .with_state(app_state.clone())
}
