use chrono::NaiveDateTime as DateTime;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter,
};
use uuid::Uuid;

use shared_shared_config::db::DB_READ;
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking::{
    Column as BookingColumn, Entity as BookingEntity,
};
use features_booking_entities::booking_window::{
    BookingWindowForCreateDto, Column, Entity, Model, ModelOptionDto,
};
use features_booking_model::booking_window::BookingWindowData;

pub struct BookingWindowMutation;

impl BookingWindowMutation {
    /// Insert a window row within a transaction. PK is `booking_id`, so the
    /// row is inserted as-is (no auto-generated id).
    pub async fn create_with_txn(
        data: BookingWindowForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let booking_id = model.booking_id;
        let active_model = model.into_active_model();
        active_model.insert(txn).await?;
        Ok(booking_id)
    }

    /// Count existing WINDOW bookings for the same resource whose time range
    /// overlaps `[starts_at, ends_at)`. Two ranges overlap iff
    /// `existing.starts_at < new.ends_at AND existing.ends_at > new.starts_at`.
    ///
    /// Runs inside the caller's transaction so the check + insert are atomic.
    /// Only considers bookings that are not CANCELLED/FAILED.
    pub async fn count_overlapping_with_txn(
        resource_type: &str,
        resource_id: Uuid,
        starts_at: DateTime,
        ends_at: DateTime,
        txn: &impl ConnectionTrait,
    ) -> Result<u64, DbErr> {
        // Step 1: active booking ids for this resource (no entity relation is
        // declared between booking and booking_window, so we resolve the id set
        // explicitly rather than via a JOIN).
        let active_ids: Vec<Uuid> = BookingEntity::find()
            .filter(BookingColumn::ResourceType.eq(resource_type))
            .filter(BookingColumn::ResourceId.eq(resource_id))
            .filter(BookingColumn::Status.is_not_in(["CANCELLED", "FAILED"]))
            .all(txn)
            .await?
            .into_iter()
            .map(|b| b.id)
            .collect();

        if active_ids.is_empty() {
            return Ok(0);
        }

        // Step 2: count windows among those bookings that overlap the new range.
        let count = Entity::find()
            .filter(Column::BookingId.is_in(active_ids))
            .filter(Column::StartsAt.lt(ends_at))
            .filter(Column::EndsAt.gt(starts_at))
            .count(txn)
            .await?;
        Ok(count)
    }
}

pub struct BookingWindowQuery;

impl BookingWindowQuery {
    pub async fn get_by_booking_id(booking_id: Uuid) -> Result<Option<BookingWindowData>, AppError> {
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
