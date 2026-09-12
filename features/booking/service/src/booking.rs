use sea_orm::{Iden, TransactionTrait};
use tracing::debug;
use uuid::Uuid;

use shared_shared_config::db::DB_WRITE;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking::Column;
use features_booking_model::booking::{
    BookingData, BookingForCreateRequest, BookingForUpdateRequest, BookingMode,
};
use features_booking_model::booking_history::{
    BookingHistoryData, BookingHistoryEntry, BookingHistoryEvent,
};
use features_booking_repo::booking::{BookingMutation, BookingQuery};
use features_booking_repo::booking_approval::BookingApprovalMutation;
use features_booking_repo::booking_capacity::BookingCapacityMutation;
use features_booking_repo::booking_dispatch::BookingDispatchMutation;
use features_booking_repo::booking_history::{BookingHistoryMutation, BookingHistoryQuery};
use features_booking_repo::booking_queue::BookingQueueMutation;
use features_booking_repo::booking_recurrence::BookingRecurrenceMutation;
use features_booking_repo::booking_window::BookingWindowMutation;

pub struct BookingService {}

impl BookingService {
    /// Create a booking and its mode-specific child row atomically.
    ///
    /// The core `bookings` row is always written. Depending on `booking_mode`,
    /// exactly one child row is written in the same transaction, after any
    /// mode-specific availability guard passes:
    /// - WINDOW    -> overlap guard on (resource_type, resource_id) time range
    /// - CAPACITY  -> counter guard against `metadata.capacity_limit`
    /// - RECURRENCE / APPROVAL / QUEUE / DISPATCH -> child row (no scarcity guard)
    pub async fn create_booking(
        booking_request: BookingForCreateRequest,
        actor_id: Option<Uuid>,
    ) -> Result<Uuid, AppError> {
        Self::validate_mode(&booking_request)?;

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        // 1. Core booking row.
        let core_dto = booking_request.to_core_dto();
        let initial_status = booking_request.status.clone();
        let booking_id = BookingMutation::create_booking_with_txn(core_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error creating booking: {:?}", e);
                AppError::Internal("Failed to create booking".to_string())
            })?;

        // 2. Mode-specific guard + child row (reserve step).
        match booking_request.booking_mode {
            BookingMode::Window => {
                let w = booking_request.window.as_ref().expect("validated present");

                // Overlap guard: only when a concrete resource unit is targeted.
                if let (Some(rtype), Some(rid)) = (
                    booking_request.resource_type.as_ref(),
                    booking_request.resource_id,
                ) {
                    let overlaps = BookingWindowMutation::count_overlapping_with_txn(
                        rtype, rid, w.starts_at, w.ends_at, &txn,
                    )
                    .await
                    .map_err(|e| {
                        debug!("Error checking window overlap: {:?}", e);
                        AppError::Internal("Failed to check availability".to_string())
                    })?;
                    if overlaps > 0 {
                        return Err(AppError::Internal(
                            "resource is already booked for an overlapping time range".to_string(),
                        ));
                    }
                }

                BookingWindowMutation::create_with_txn(w.to_dto(booking_id), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking window: {:?}", e);
                        AppError::Internal("Failed to create booking window".to_string())
                    })?;
            }
            BookingMode::Capacity => {
                let c = booking_request.capacity.as_ref().expect("validated present");

                // Counter guard: enforce only if the caller declares a limit
                // via metadata.capacity_limit.
                if let Some(limit) = Self::metadata_i64(&booking_request, "capacity_limit") {
                    let booked = BookingCapacityMutation::sum_booked_quantity_with_txn(
                        c.container_id,
                        &txn,
                    )
                    .await
                    .map_err(|e| {
                        debug!("Error summing capacity: {:?}", e);
                        AppError::Internal("Failed to check capacity".to_string())
                    })?;
                    if booked + c.quantity as i64 > limit {
                        return Err(AppError::Internal(
                            "capacity limit exceeded for container".to_string(),
                        ));
                    }
                }

                BookingCapacityMutation::create_with_txn(c.to_dto(booking_id), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking capacity: {:?}", e);
                        AppError::Internal("Failed to create booking capacity".to_string())
                    })?;
            }
            // Phase 3 modes: core row only for now.
            BookingMode::Recurrence => {
                let r = booking_request
                    .recurrence
                    .as_ref()
                    .expect("validated present");
                BookingRecurrenceMutation::create_with_txn(r.to_dto(booking_id), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking recurrence: {:?}", e);
                        AppError::Internal("Failed to create booking recurrence".to_string())
                    })?;
            }
            BookingMode::Approval => {
                let a = booking_request
                    .approval
                    .as_ref()
                    .expect("validated present");
                BookingApprovalMutation::create_with_txn(a.to_dto(booking_id), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking approval: {:?}", e);
                        AppError::Internal("Failed to create booking approval".to_string())
                    })?;
            }
            BookingMode::Queue => {
                let q = booking_request.queue.as_ref().expect("validated present");
                // Assign the next position in the queue within the transaction.
                let position =
                    BookingQueueMutation::next_position_with_txn(&q.queue_key, &txn)
                        .await
                        .map_err(|e| {
                            debug!("Error computing queue position: {:?}", e);
                            AppError::Internal("Failed to assign queue position".to_string())
                        })?;
                BookingQueueMutation::create_with_txn(q.to_dto(booking_id, position), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking queue: {:?}", e);
                        AppError::Internal("Failed to create booking queue".to_string())
                    })?;
            }
            BookingMode::Dispatch => {
                let d = booking_request
                    .dispatch
                    .as_ref()
                    .expect("validated present");
                BookingDispatchMutation::create_with_txn(d.to_dto(booking_id), &txn)
                    .await
                    .map_err(|e| {
                        debug!("Error creating booking dispatch: {:?}", e);
                        AppError::Internal("Failed to create booking dispatch".to_string())
                    })?;
            }
        }

        // Record the CREATED event in the same transaction.
        let entry = BookingHistoryEntry::new(booking_id, BookingHistoryEvent::CREATED)
            .with_status_change(None, Some(initial_status))
            .with_actor(actor_id);
        BookingHistoryMutation::append_with_txn(entry.into(), &txn)
            .await
            .map_err(|e| {
                debug!("Error recording booking history (CREATED): {:?}", e);
                AppError::Internal("Failed to record booking history".to_string())
            })?;

        txn.commit().await.map_err(|e| {
            debug!("Error committing booking transaction: {:?}", e);
            AppError::Internal("Failed to commit booking".to_string())
        })?;

        Ok(booking_id)
    }

    /// Read an integer value from the request's `metadata` JSON object.
    fn metadata_i64(req: &BookingForCreateRequest, key: &str) -> Option<i64> {
        req.metadata
            .as_ref()
            .and_then(|m| m.get(key))
            .and_then(|v| v.as_i64())
    }

    /// Validate that the sub-block matching the requested mode is present and
    /// internally consistent.
    fn validate_mode(req: &BookingForCreateRequest) -> Result<(), AppError> {
        match req.booking_mode {
            BookingMode::Window => {
                let w = req.window.as_ref().ok_or_else(|| {
                    AppError::Internal("WINDOW mode requires a `window` block".to_string())
                })?;
                if w.ends_at <= w.starts_at {
                    return Err(AppError::Internal(
                        "window.ends_at must be after window.starts_at".to_string(),
                    ));
                }
                if let Some(size) = w.party_size {
                    if size <= 0 {
                        return Err(AppError::Internal(
                            "window.party_size must be positive".to_string(),
                        ));
                    }
                }
                Ok(())
            }
            BookingMode::Capacity => {
                let c = req.capacity.as_ref().ok_or_else(|| {
                    AppError::Internal("CAPACITY mode requires a `capacity` block".to_string())
                })?;
                if c.quantity <= 0 {
                    return Err(AppError::Internal(
                        "capacity.quantity must be positive".to_string(),
                    ));
                }
                Ok(())
            }
            BookingMode::Recurrence => {
                let r = req.recurrence.as_ref().ok_or_else(|| {
                    AppError::Internal("RECURRENCE mode requires a `recurrence` block".to_string())
                })?;
                if let Some(valid_to) = r.valid_to {
                    if valid_to <= r.valid_from {
                        return Err(AppError::Internal(
                            "recurrence.valid_to must be after valid_from".to_string(),
                        ));
                    }
                }
                Ok(())
            }
            BookingMode::Approval => {
                req.approval.as_ref().ok_or_else(|| {
                    AppError::Internal("APPROVAL mode requires an `approval` block".to_string())
                })?;
                Ok(())
            }
            BookingMode::Queue => {
                let q = req.queue.as_ref().ok_or_else(|| {
                    AppError::Internal("QUEUE mode requires a `queue` block".to_string())
                })?;
                if q.queue_key.trim().is_empty() {
                    return Err(AppError::Internal(
                        "queue.queue_key must not be empty".to_string(),
                    ));
                }
                Ok(())
            }
            BookingMode::Dispatch => {
                req.dispatch.as_ref().ok_or_else(|| {
                    AppError::Internal("DISPATCH mode requires a `dispatch` block".to_string())
                })?;
                Ok(())
            }
        }
    }

    pub async fn get_booking_by_id(
        booking_id: Uuid,
        query_params: &QueryParams,
    ) -> Result<BookingData, AppError> {
        BookingQuery::get_booking_by_id(booking_id, query_params).await
    }

    pub async fn get_bookings_by_status(
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let param: FilterParam<String> = FilterParam {
            name: Column::Status.to_string(),
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(param)].into();
        BookingQuery::get_bookings(pagination, order, &filters, &QueryParams::default()).await
    }

    pub async fn get_bookings_by_user(
        user_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let param: FilterParam<Uuid> = FilterParam {
            name: Column::UserId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(user_id),
            raw_value: user_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::Uuid(param)].into();
        BookingQuery::get_bookings(pagination, order, &filters, &QueryParams::default()).await
    }

    pub async fn get_bookings(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
        query_params: &QueryParams,
    ) -> Result<QueryResult<BookingData>, AppError> {
        BookingQuery::get_bookings(pagination, order, filters, query_params).await
    }

    pub async fn update_booking(
        booking_id: Uuid,
        booking_request: BookingForUpdateRequest,
        actor_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        // Read current state to diff status / payment_status for history.
        let current = BookingQuery::get_booking_by_id(booking_id, &QueryParams::default()).await?;
        let prev_status = current.status.clone();
        let prev_payment = current.payment_status.clone();

        let new_status = booking_request.status.clone();
        let new_payment = booking_request.payment_status.clone();

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        let updated = BookingMutation::update_booking_with_txn(
            booking_id,
            booking_request.into(),
            &txn,
        )
        .await
        .map_err(|e| {
            debug!("Error updating booking: {:?}", e);
            AppError::Internal("Failed to update booking".to_string())
        })?;

        if !updated {
            txn.rollback().await.ok();
            return Ok(false);
        }

        // Record a STATUS_CHANGED event if status actually changed.
        if let Some(to) = new_status {
            if Some(&to) != prev_status.as_ref() {
                let entry = BookingHistoryEntry::new(booking_id, BookingHistoryEvent::STATUS_CHANGED)
                    .with_status_change(prev_status.clone(), Some(to))
                    .with_actor(actor_id);
                Self::append_history(entry, &txn).await?;
            }
        }

        // Record a PAYMENT_UPDATED event if payment_status actually changed.
        if let Some(to_payment) = new_payment {
            if Some(&to_payment) != prev_payment.as_ref() {
                let entry = BookingHistoryEntry::new(booking_id, BookingHistoryEvent::PAYMENT_UPDATED)
                    .with_actor(actor_id)
                    .with_note(Some(format!(
                        "payment_status: {} -> {}",
                        prev_payment.unwrap_or_default(),
                        to_payment
                    )));
                Self::append_history(entry, &txn).await?;
            }
        }

        txn.commit().await.map_err(|e| {
            debug!("Error committing booking update: {:?}", e);
            AppError::Internal("Failed to commit booking update".to_string())
        })?;

        Ok(true)
    }

    /// Soft-delete: mark the booking CANCELLED and record a CANCELLED event.
    /// The row (and its history) is preserved for the audit trail.
    pub async fn delete_booking(
        booking_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        let current = BookingQuery::get_booking_by_id(booking_id, &QueryParams::default()).await?;
        let prev_status = current.status.clone();

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        let cancel_dto = BookingForUpdateRequest {
            booking_type: None,
            resource_type: None,
            resource_id: None,
            total_amount: None,
            currency: None,
            status: Some("CANCELLED".to_string()),
            payment_id: None,
            payment_status: None,
            metadata: None,
            confirmed_at: None,
        };
        let updated =
            BookingMutation::update_booking_with_txn(booking_id, cancel_dto.into(), &txn)
                .await
                .map_err(|e| {
                    debug!("Error cancelling booking: {:?}", e);
                    AppError::Internal("Failed to cancel booking".to_string())
                })?;

        if !updated {
            txn.rollback().await.ok();
            return Ok(false);
        }

        let entry = BookingHistoryEntry::new(booking_id, BookingHistoryEvent::CANCELLED)
            .with_status_change(prev_status, Some("CANCELLED".to_string()))
            .with_actor(actor_id);
        Self::append_history(entry, &txn).await?;

        txn.commit().await.map_err(|e| {
            debug!("Error committing booking cancellation: {:?}", e);
            AppError::Internal("Failed to commit booking cancellation".to_string())
        })?;

        Ok(true)
    }

    /// Paginated lifecycle history for a booking, oldest first.
    pub async fn get_booking_history(
        booking_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<BookingHistoryData>, AppError> {
        let param: FilterParam<Uuid> = FilterParam {
            name: features_booking_entities::booking_history::Column::BookingId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(booking_id),
            raw_value: booking_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::Uuid(param)].into();
        BookingHistoryQuery::list(pagination, order, &filters).await
    }

    /// Append a history entry, mapping errors uniformly.
    async fn append_history(
        entry: BookingHistoryEntry,
        txn: &impl sea_orm::ConnectionTrait,
    ) -> Result<(), AppError> {
        BookingHistoryMutation::append_with_txn(entry.into(), txn)
            .await
            .map_err(|e| {
                debug!("Error recording booking history: {:?}", e);
                AppError::Internal("Failed to record booking history".to_string())
            })?;
        Ok(())
    }
}
