use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder,
};
use uuid::Uuid;

use shared_shared_config::db::DB_READ;
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking_queue::{
    BookingQueueForCreateDto, Column, Entity, Model, ModelOptionDto,
};
use features_booking_model::booking_queue::BookingQueueData;

pub struct BookingQueueMutation;

impl BookingQueueMutation {
    /// Insert a queue row within a transaction. PK is `booking_id`.
    pub async fn create_with_txn(
        data: BookingQueueForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let booking_id = model.booking_id;
        let active_model = model.into_active_model();
        active_model.insert(txn).await?;
        Ok(booking_id)
    }

    /// Compute the next queue position for a `queue_key` (max existing + 1,
    /// starting at 1). Runs in the transaction so concurrent inserts serialize.
    pub async fn next_position_with_txn(
        queue_key: &str,
        txn: &impl ConnectionTrait,
    ) -> Result<i32, DbErr> {
        let last = Entity::find()
            .filter(Column::QueueKey.eq(queue_key))
            .order_by_desc(Column::Position)
            .one(txn)
            .await?;
        let next = last.and_then(|m| m.position).unwrap_or(0) + 1;
        Ok(next)
    }
}

pub struct BookingQueueQuery;

impl BookingQueueQuery {
    pub async fn get_by_booking_id(booking_id: Uuid) -> Result<Option<BookingQueueData>, AppError> {
        let db = DB_READ.get().expect("DB_READ not initialized");
        let model = Entity::find_by_id(booking_id)
            .one(db.as_ref())
            .await
            .map_err(|_| AppError::Unknown)?;
        Ok(model.map(|m| {
            let opt: ModelOptionDto = m.into();
            opt.into()
        }))
    }
}
