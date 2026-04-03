use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_wallet_entities::p2p_transfer::{
    ModelOptionDto, P2pTransferForCreateDto, P2pTransferForUpdateDto,
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct P2pTransferForCreateRequest {
    pub from_wallet_id: Uuid,
    pub to_wallet_id: Uuid,
    #[validate(range(
        min = 0.01,
        code = "p2p_transfer_amount_positive",
        message = "amount must be greater than 0"
    ))]
    pub amount: f32,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct P2pTransferForUpdateRequest {
    pub status: Option<String>,
    pub completed_at: Option<DateTime>,
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct P2pTransferData {
    pub id: Option<Uuid>,
    pub from_wallet_id: Option<Uuid>,
    pub to_wallet_id: Option<Uuid>,
    pub amount: Option<f32>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,
}

impl Into<P2pTransferData> for ModelOptionDto {
    fn into(self) -> P2pTransferData {
        P2pTransferData {
            id: self.id,
            from_wallet_id: self.from_wallet_id,
            to_wallet_id: self.to_wallet_id,
            amount: self.amount,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            completed_at: self.completed_at,
        }
    }
}

impl Into<P2pTransferForCreateDto> for P2pTransferForCreateRequest {
    fn into(self) -> P2pTransferForCreateDto {
        P2pTransferForCreateDto {
            from_wallet_id: self.from_wallet_id,
            to_wallet_id: self.to_wallet_id,
            amount: self.amount,
            status: "PENDING".to_string(),
            completed_at: DateTime::default(),
        }
    }
}

impl Into<P2pTransferForUpdateDto> for P2pTransferForUpdateRequest {
    fn into(self) -> P2pTransferForUpdateDto {
        P2pTransferForUpdateDto {
            status: self.status,
            completed_at: self.completed_at,
        }
    }
}
