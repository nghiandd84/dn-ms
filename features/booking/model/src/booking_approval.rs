use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use features_booking_entities::booking_approval::{
    BookingApprovalForCreateDto, Model, ModelOptionDto,
};

/// APPROVAL-mode input block. The workflow always starts in `PENDING`; the
/// client may optionally set how long the request is held awaiting a decision.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingApprovalInput {
    pub hold_expires_at: Option<DateTime>,
}

impl BookingApprovalInput {
    pub fn to_dto(&self, booking_id: Uuid) -> BookingApprovalForCreateDto {
        BookingApprovalForCreateDto {
            booking_id,
            approval_status: "PENDING".to_string(),
            hold_expires_at: self.hold_expires_at,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct BookingApprovalData {
    pub booking_id: Option<Uuid>,
    pub approval_status: Option<String>,
    pub approver_id: Option<Uuid>,
    pub requested_at: Option<DateTime>,
    pub decided_at: Option<DateTime>,
    pub hold_expires_at: Option<DateTime>,
    pub reason: Option<String>,
}

impl Into<BookingApprovalData> for ModelOptionDto {
    fn into(self) -> BookingApprovalData {
        BookingApprovalData {
            booking_id: self.booking_id,
            approval_status: self.approval_status,
            approver_id: self.approver_id.flatten(),
            requested_at: self.requested_at,
            decided_at: self.decided_at.flatten(),
            hold_expires_at: self.hold_expires_at.flatten(),
            reason: self.reason.flatten(),
        }
    }
}

impl From<Model> for BookingApprovalData {
    fn from(m: Model) -> Self {
        BookingApprovalData {
            booking_id: Some(m.booking_id),
            approval_status: Some(m.approval_status),
            approver_id: m.approver_id,
            requested_at: Some(m.requested_at),
            decided_at: m.decided_at,
            hold_expires_at: m.hold_expires_at,
            reason: m.reason,
        }
    }
}
