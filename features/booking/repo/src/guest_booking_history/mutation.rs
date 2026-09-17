use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, DbErr, IntoActiveModel};
use uuid::Uuid;

use features_booking_entities::guest_booking_history::{GuestBookingHistoryForCreateDto, Model};

pub struct GuestBookingHistoryMutation;

impl GuestBookingHistoryMutation {
    /// Append a history event within an existing transaction, returning its id.
    /// Append-only: the log has no update or delete operations.
    pub async fn append_with_txn(
        data: GuestBookingHistoryForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let mut active_model = model.into_active_model();
        // id is DB-generated; created_at is set by before_save on insert.
        active_model.id = NotSet;
        let result = active_model.insert(txn).await?;
        Ok(result.id)
    }
}
