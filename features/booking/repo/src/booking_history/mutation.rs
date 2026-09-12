use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, DbErr, IntoActiveModel};
use uuid::Uuid;

use features_booking_entities::booking_history::{BookingHistoryForCreateDto, Model};

pub struct BookingHistoryMutation;

impl BookingHistoryMutation {
    /// Append a history event within an existing transaction, returning its id.
    /// Append-only: the log has no update or delete operations.
    pub async fn append_with_txn(
        data: BookingHistoryForCreateDto,
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
