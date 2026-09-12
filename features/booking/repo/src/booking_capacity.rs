use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use uuid::Uuid;

use shared_shared_config::db::DB_READ;
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking::{
    Column as BookingColumn, Entity as BookingEntity,
};
use features_booking_entities::booking_capacity::{
    BookingCapacityForCreateDto, Column, Entity, Model, ModelOptionDto,
};
use features_booking_model::booking_capacity::BookingCapacityData;

pub struct BookingCapacityMutation;

impl BookingCapacityMutation {
    /// Insert a capacity row within a transaction. PK is `booking_id`.
    pub async fn create_with_txn(
        data: BookingCapacityForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let booking_id = model.booking_id;
        let active_model = model.into_active_model();
        active_model.insert(txn).await?;
        Ok(booking_id)
    }

    /// Sum the `quantity` already booked against a container across all active
    /// (not CANCELLED/FAILED) bookings. Used to enforce a capacity limit.
    ///
    /// Runs inside the caller's transaction so the check + insert are atomic.
    pub async fn sum_booked_quantity_with_txn(
        container_id: Uuid,
        txn: &impl ConnectionTrait,
    ) -> Result<i64, DbErr> {
        // Resolve active booking ids (no declared entity relation, so resolve
        // the id set explicitly instead of via JOIN).
        let active_ids: Vec<Uuid> = BookingEntity::find()
            .filter(BookingColumn::Status.is_not_in(["CANCELLED", "FAILED"]))
            .all(txn)
            .await?
            .into_iter()
            .map(|b| b.id)
            .collect();

        if active_ids.is_empty() {
            return Ok(0);
        }

        let rows = Entity::find()
            .filter(Column::ContainerId.eq(container_id))
            .filter(Column::BookingId.is_in(active_ids))
            .all(txn)
            .await?;
        let total: i64 = rows.iter().map(|r| r.quantity as i64).sum();
        Ok(total)
    }
}

pub struct BookingCapacityQuery;

impl BookingCapacityQuery {
    pub async fn get_by_booking_id(
        booking_id: Uuid,
    ) -> Result<Option<BookingCapacityData>, AppError> {
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
