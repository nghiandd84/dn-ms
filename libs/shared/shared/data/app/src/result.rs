use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_macro::Response;

use crate::error::AppError;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Serialize, Clone, ToSchema, Response)]
pub struct OkUuid {
    pub ok: bool,
    pub id: Option<Uuid>,
}

#[derive(Serialize, Clone, ToSchema, Response)]
pub struct OkI32 {
    pub ok: bool,
    pub id: Option<i32>,
}
