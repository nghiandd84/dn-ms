use chrono::Utc;
use rand::RngCore;
use sea_orm::{Iden, TransactionTrait};
use sha2::{Digest, Sha256};
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_config::db::DB_WRITE;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_booking_model::booking::BookingMode;
use features_booking_model::booking_history::{BookingHistoryEntry, BookingHistoryEvent};
use features_booking_model::guest_booking::{
    GuestBookingAdminForCreateRequest, GuestBookingConfirmRequest, GuestBookingData,
    GuestBookingForCreateRequest, GuestBookingForUpdateRequest, GuestBookingStatus,
};
use features_booking_model::guest_booking_history::{
    GuestBookingHistoryData, GuestBookingHistoryEntry, GuestBookingHistoryEvent,
};
use features_booking_stream::{BookingMessage, GuestBookingConfirmTokenMessage};

use features_booking_entities::booking::BookingForCreateDto;
use features_booking_entities::booking_capacity::BookingCapacityForCreateDto;
use features_booking_entities::booking_item::BookingItemForCreateDto;

use features_booking_repo::booking::BookingMutation;
use features_booking_repo::booking_capacity::BookingCapacityMutation;
use features_booking_repo::booking_history::BookingHistoryMutation;
use features_booking_repo::booking_item::BookingItemMutation;
use features_booking_repo::guest_booking::{GuestBookingMutation, GuestBookingQuery};
use features_booking_repo::guest_booking_history::{
    GuestBookingHistoryMutation, GuestBookingHistoryQuery,
};
use features_booking_repo::guest_booking_item::{GuestBookingItemMutation, GuestBookingItemQuery};

pub struct GuestBookingService {}

impl GuestBookingService {
    /// Minutes a guest has to confirm a booking before it expires.
    pub const CONFIRM_WINDOW_MINUTES: i64 = 20;

    /// Minutes a guest has to complete payment (promotion) after confirming.
    pub const PAYMENT_WINDOW_MINUTES: i64 = 60;

    /// Maximum seats (booking items) allowed in a single guest booking.
    pub const MAX_SEATS_PER_BOOKING: usize = 10;

    /// Maximum active PENDING guest bookings allowed per email within the
    /// throttle window below.
    pub const MAX_ACTIVE_PENDING_PER_EMAIL: u64 = 5;

    /// Rolling window (minutes) for the per-email active-PENDING throttle.
    pub const PENDING_THROTTLE_WINDOW_MINUTES: i64 = 60;
    /// Create a guest booking with exactly one item per selected seat, in a
    /// single transaction. `total_amount` is computed from the seat prices and a
    /// unique `booking_reference` is generated.
    ///
    /// A one-time confirmation token is generated: only its SHA-256 hash is
    /// stored. The plaintext token is NOT returned over HTTP — after the booking
    /// is committed, it is published to the booking Kafka topic (in a
    /// `GuestBookingConfirmToken` event) so a downstream consumer can email it to
    /// the guest. This authorizes the later unauthenticated confirm step without
    /// ever exposing the token to callers who merely know the booking id.
    pub async fn create_guest_booking(
        request: GuestBookingForCreateRequest,
        producer: &Producer,
    ) -> Result<Uuid, AppError> {
        let mut request = request;
        if request.seats.is_empty() {
            return Err(AppError::Internal(
                "at least one seat is required".to_string(),
            ));
        }
        if request.seats.len() > Self::MAX_SEATS_PER_BOOKING {
            return Err(AppError::Internal(format!(
                "too many seats: max {} per booking",
                Self::MAX_SEATS_PER_BOOKING
            )));
        }

        // Validate the site origin against the allowlist BEFORE doing any work.
        // This prevents confirmation emails from linking to arbitrary domains.
        let site_origin = Self::normalize_origin(&request.site_origin);
        if !Self::is_origin_allowed(&site_origin) {
            debug!(
                "Rejected guest booking: site_origin not allowed: {}",
                site_origin
            );
            return Err(AppError::Internal(
                "site_origin is not an allowed site".to_string(),
            ));
        }
        // Persist the canonical (normalized) origin.
        request.site_origin = site_origin.clone();

        // Per-email throttle: cap active PENDING bookings in a rolling window to
        // limit abuse (spam bookings / email bombing) of this public endpoint.
        let since = Utc::now().naive_utc()
            - chrono::Duration::minutes(Self::PENDING_THROTTLE_WINDOW_MINUTES);
        let active_pending =
            GuestBookingQuery::count_active_pending_by_email_since(&request.guest_email, since)
                .await?;
        if active_pending >= Self::MAX_ACTIVE_PENDING_PER_EMAIL {
            debug!(
                "Rejected guest booking: {} active pending for {}",
                active_pending, request.guest_email
            );
            return Err(AppError::Internal(
                "too many pending bookings for this email; please confirm or wait".to_string(),
            ));
        }

        let total_amount: f32 = request.seats.iter().map(|s| s.price).sum();
        let booking_reference = Self::generate_reference("GBK");

        // Generate the one-time confirmation token; store only its hash.
        let confirm_token = Self::generate_token();
        let confirm_token_hash = Self::hash_token(&confirm_token);

        // Confirmation window: the guest must confirm before this instant.
        let expires_at =
            Utc::now().naive_utc() + chrono::Duration::minutes(Self::CONFIRM_WINDOW_MINUTES);

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        // 1. Core guest-booking row (status PENDING).
        let core_dto = request.to_core_dto(
            total_amount,
            booking_reference.clone(),
            confirm_token_hash,
            expires_at,
        );
        let guest_booking_id = GuestBookingMutation::create_guest_booking_with_txn(core_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error creating guest booking: {:?}", e);
                AppError::Internal("Failed to create guest booking".to_string())
            })?;

        // 2. One item per seat.
        for seat in &request.seats {
            GuestBookingItemMutation::create_guest_booking_item_with_txn(
                seat.to_dto(guest_booking_id),
                &txn,
            )
            .await
            .map_err(|e| {
                debug!("Error creating guest booking item: {:?}", e);
                AppError::Internal("Failed to create guest booking item".to_string())
            })?;
        }

        // 3. Record the CREATED lifecycle event in the same transaction.
        let history =
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::CREATED)
                .with_status_change(None, Some(GuestBookingStatus::PENDING.to_string()))
                .with_note(Some("guest booking created (public)".to_string()));
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;

        // 3. Publish the confirm token out-of-band (AFTER commit). A failure to
        //    publish must not roll back the booking, but is surfaced as an error
        //    so the caller/consumer can react (e.g. retry delivery).
        let confirm_url = Self::build_confirm_url(
            &site_origin,
            &request.confirm_path,
            guest_booking_id,
            &confirm_token,
        );
        let payload = BookingMessage::GuestBookingConfirmToken {
            message: GuestBookingConfirmTokenMessage {
                guest_booking_id,
                resource_type: request.resource_type.clone(),
                resource_id: request.resource_id,
                external_ref: request.external_ref.clone(),
                guest_email: request.guest_email.clone(),
                guest_name: request.guest_name.clone(),
                booking_reference,
                confirm_token,
                confirm_url,
                expires_at: expires_at.and_utc().to_rfc3339(),
            },
        };
        let message = ProducerMessage {
            payload,
            key: Some(guest_booking_id.to_string()),
        };
        producer.send(&message).await.map_err(|e| {
            debug!(
                "Error sending guest booking confirm-token event to Kafka: {:?}",
                e.reason
            );
            AppError::Unknown
        })?;

        Ok(guest_booking_id)
    }

    pub async fn get_guest_booking_by_id(
        guest_booking_id: Uuid,
        query_params: &QueryParams,
    ) -> Result<GuestBookingData, AppError> {
        GuestBookingQuery::get_guest_booking_by_id(guest_booking_id, query_params).await
    }

    pub async fn get_guest_bookings(
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
        query_params: &QueryParams,
    ) -> Result<QueryResult<GuestBookingData>, AppError> {
        GuestBookingQuery::get_guest_bookings(pagination, order, filters, query_params).await
    }

    /// Paginated lifecycle history for a guest booking.
    pub async fn get_guest_booking_history(
        guest_booking_id: Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<GuestBookingHistoryData>, AppError> {
        let param: FilterParam<Uuid> = FilterParam {
            name: features_booking_entities::guest_booking_history::Column::GuestBookingId
                .to_string(),
            operator: FilterOperator::Equal,
            value: Some(guest_booking_id),
            raw_value: guest_booking_id.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::Uuid(param)].into();
        GuestBookingHistoryQuery::list(pagination, order, &filters).await
    }

    /// Append a guest booking history entry within an existing transaction,
    /// mapping errors uniformly.
    async fn append_history(
        entry: GuestBookingHistoryEntry,
        txn: &impl sea_orm::ConnectionTrait,
    ) -> Result<(), AppError> {
        GuestBookingHistoryMutation::append_with_txn(entry.into(), txn)
            .await
            .map_err(|e| {
                debug!("Error recording guest booking history: {:?}", e);
                AppError::Internal("Failed to record guest booking history".to_string())
            })?;
        Ok(())
    }

    /// Admin: create a guest booking record directly, bypassing the public
    /// confirm-token / Kafka email flow. The administrator supplies the status
    /// and (optionally) a booking reference and expiry. No confirmation token is
    /// generated or stored. Returns the new guest booking id.
    pub async fn create_guest_booking_admin(
        request: GuestBookingAdminForCreateRequest,
        actor_id: Option<Uuid>,
    ) -> Result<Uuid, AppError> {
        let booking_reference = request
            .booking_reference
            .clone()
            .filter(|r| !r.trim().is_empty())
            .unwrap_or_else(|| Self::generate_reference("GBK"));

        let expires_at = request.expires_at.unwrap_or_else(|| {
            Utc::now().naive_utc() + chrono::Duration::minutes(Self::CONFIRM_WINDOW_MINUTES)
        });

        let status = request.status.clone();
        let core_dto = request.to_core_dto(booking_reference, expires_at);

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        let guest_booking_id = GuestBookingMutation::create_guest_booking_with_txn(core_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error creating guest booking (admin): {:?}", e);
                AppError::Internal("Failed to create guest booking".to_string())
            })?;

        // Record the CREATED lifecycle event in the same transaction.
        let history =
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::CREATED)
                .with_status_change(None, Some(status))
                .with_actor(actor_id)
                .with_note(Some("guest booking created by administrator".to_string()));
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;

        Ok(guest_booking_id)
    }

    /// Admin: update an existing guest booking record and record the change in
    /// the history log. A status change is recorded as STATUS_CHANGED (with
    /// from/to), otherwise as UPDATED. Returns false if the booking does not
    /// exist.
    pub async fn update_guest_booking_admin(
        guest_booking_id: Uuid,
        request: GuestBookingForUpdateRequest,
        actor_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        // Load current state to detect status transitions for the history log.
        let existing = GuestBookingQuery::get_by_id_with_txn(guest_booking_id, &txn)
            .await
            .map_err(|_| AppError::Unknown)?;
        let Some(existing) = existing else {
            txn.rollback().await.ok();
            return Ok(false);
        };
        let prev_status = existing.status.clone();
        let new_status = request.status.clone();

        let updated = GuestBookingMutation::update_guest_booking_with_txn(
            guest_booking_id,
            request.into(),
            &txn,
        )
        .await
        .map_err(|e| {
            debug!("Error updating guest booking (admin): {:?}", e);
            AppError::Internal("Failed to update guest booking".to_string())
        })?;

        if !updated {
            txn.rollback().await.ok();
            return Ok(false);
        }

        // Choose the event type based on whether the status actually changed.
        let status_changed = matches!(&new_status, Some(s) if *s != prev_status);
        let history = if status_changed {
            GuestBookingHistoryEntry::new(
                guest_booking_id,
                GuestBookingHistoryEvent::STATUS_CHANGED,
            )
            .with_status_change(Some(prev_status), new_status)
            .with_actor(actor_id)
            .with_note(Some("guest booking updated by administrator".to_string()))
        } else {
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::UPDATED)
                .with_actor(actor_id)
                .with_note(Some("guest booking updated by administrator".to_string()))
        };
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;
        Ok(true)
    }

    /// Admin: cancel a guest booking (soft delete) and record a CANCELLED event.
    /// The row (and its history) is preserved for the audit trail. Returns false
    /// if the booking does not exist.
    pub async fn delete_guest_booking_admin(
        guest_booking_id: Uuid,
        actor_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        let existing = GuestBookingQuery::get_by_id_with_txn(guest_booking_id, &txn)
            .await
            .map_err(|_| AppError::Unknown)?;
        let Some(existing) = existing else {
            txn.rollback().await.ok();
            return Ok(false);
        };
        let prev_status = existing.status.clone();

        let cancel_update = GuestBookingForUpdateRequest {
            guest_email: None,
            guest_name: None,
            total_amount: None,
            currency: None,
            status: Some(GuestBookingStatus::CANCELLED.to_string()),
            booking_reference: None,
            metadata: None,
            promoted_booking_id: None,
            confirmed_at: None,
            payment_expires_at: None,
        };
        let updated = GuestBookingMutation::update_guest_booking_with_txn(
            guest_booking_id,
            cancel_update.into(),
            &txn,
        )
        .await
        .map_err(|e| {
            debug!("Error cancelling guest booking (admin): {:?}", e);
            AppError::Internal("Failed to cancel guest booking".to_string())
        })?;

        if !updated {
            txn.rollback().await.ok();
            return Ok(false);
        }

        let history =
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::CANCELLED)
                .with_status_change(
                    Some(prev_status),
                    Some(GuestBookingStatus::CANCELLED.to_string()),
                )
                .with_actor(actor_id)
                .with_note(Some("guest booking cancelled by administrator".to_string()));
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;
        Ok(true)
    }

    /// Confirm a PENDING guest booking: set status CONFIRMED + confirmed_at.
    ///
    /// Requires the one-time `confirm_token` issued at creation. The presented
    /// token is hashed and compared in constant time against the stored hash;
    /// a mismatch (or a booking with no token) is rejected. This is what
    /// authorizes the otherwise-unauthenticated confirm.
    pub async fn confirm_guest_booking(
        guest_booking_id: Uuid,
        request: GuestBookingConfirmRequest,
    ) -> Result<bool, AppError> {
        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        let existing = GuestBookingQuery::get_by_id_with_txn(guest_booking_id, &txn)
            .await
            .map_err(|_| AppError::Unknown)?
            .ok_or_else(|| AppError::Internal("guest booking not found".to_string()))?;

        // Authorize via the one-time confirmation token (constant-time check).
        let stored_hash = existing.confirm_token_hash.as_deref().ok_or_else(|| {
            AppError::Auth(shared_shared_data_error::auth::AuthError::InsufficientPermission)
        })?;
        let presented_hash = Self::hash_token(&request.confirm_token);
        if !Self::constant_time_eq(stored_hash.as_bytes(), presented_hash.as_bytes()) {
            return Err(AppError::Auth(
                shared_shared_data_error::auth::AuthError::InsufficientPermission,
            ));
        }

        if existing.status == GuestBookingStatus::PROMOTED {
            return Err(AppError::Internal(
                "guest booking has already been promoted".to_string(),
            ));
        }
        if existing.status == GuestBookingStatus::CANCELLED {
            return Err(AppError::Internal("guest booking is cancelled".to_string()));
        }
        if existing.status == GuestBookingStatus::EXPIRED {
            return Err(AppError::Internal("guest booking has expired".to_string()));
        }

        // Enforce the confirmation window. If the deadline has passed while the
        // booking is still PENDING, mark it EXPIRED and reject the confirm.
        if Utc::now().naive_utc() > existing.expires_at {
            let expire_update = GuestBookingForUpdateRequest {
                guest_email: None,
                guest_name: None,
                total_amount: None,
                currency: None,
                status: Some(GuestBookingStatus::EXPIRED.to_string()),
                booking_reference: None,
                metadata: None,
                promoted_booking_id: None,
                confirmed_at: None,
                payment_expires_at: None,
            };
            // Best-effort status flip; ignore failure (the reject is what matters).
            let _ = GuestBookingMutation::update_guest_booking_with_txn(
                guest_booking_id,
                expire_update.into(),
                &txn,
            )
            .await;
            // Record the EXPIRED transition (best-effort, same txn).
            let history =
                GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::EXPIRED)
                    .with_status_change(
                        Some(existing.status.clone()),
                        Some(GuestBookingStatus::EXPIRED.to_string()),
                    )
                    .with_note(Some("confirmation window elapsed".to_string()));
            let _ = Self::append_history(history, &txn).await;
            txn.commit().await.map_err(|_| AppError::Unknown)?;
            return Err(AppError::Internal(
                "guest booking has expired; confirmation window elapsed".to_string(),
            ));
        }

        let now = Utc::now().naive_utc();
        // Start the payment window: the guest must pay (promote) before this.
        let payment_expires_at = now + chrono::Duration::minutes(Self::PAYMENT_WINDOW_MINUTES);
        let update = GuestBookingForUpdateRequest {
            guest_email: None,
            guest_name: None,
            total_amount: None,
            currency: None,
            status: Some(GuestBookingStatus::CONFIRMED.to_string()),
            booking_reference: None,
            metadata: None,
            promoted_booking_id: None,
            confirmed_at: Some(now),
            payment_expires_at: Some(payment_expires_at),
        };
        let ok = GuestBookingMutation::update_guest_booking_with_txn(
            guest_booking_id,
            update.into(),
            &txn,
        )
        .await
        .map_err(|e| {
            debug!("Error confirming guest booking: {:?}", e);
            AppError::Internal("Failed to confirm guest booking".to_string())
        })?;

        // Record the CONFIRMED lifecycle event in the same transaction.
        let history =
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::CONFIRMED)
                .with_status_change(
                    Some(existing.status.clone()),
                    Some(GuestBookingStatus::CONFIRMED.to_string()),
                )
                .with_note(Some("guest confirmed booking".to_string()));
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;
        Ok(ok)
    }

    /// Promote a CONFIRMED guest booking into a real booking.
    ///
    /// In a single transaction: creates a core `bookings` row carrying the guest
    /// booking's `booking_type` / `booking_mode` and polymorphic target
    /// (`resource_type` / `resource_id`); for CAPACITY bookings also creates the
    /// capacity child row (container = `resource_id`); one `booking_item` per
    /// guest seat item; a `booking_history` CREATED entry; then marks the guest
    /// booking PROMOTED and records the new booking id. Returns the new booking
    /// id.
    pub async fn promote_guest_booking(
        guest_booking_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Uuid, AppError> {
        let user_id = user_id
            .ok_or_else(|| AppError::Internal("authenticated user_id is required".to_string()))?;

        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        // 1. Load + validate guest booking.
        let guest = GuestBookingQuery::get_by_id_with_txn(guest_booking_id, &txn)
            .await
            .map_err(|_| AppError::Unknown)?
            .ok_or_else(|| AppError::Internal("guest booking not found".to_string()))?;

        if guest.status != GuestBookingStatus::CONFIRMED {
            return Err(AppError::Internal(
                "guest booking must be CONFIRMED before promotion".to_string(),
            ));
        }
        if guest.promoted_booking_id.is_some() {
            return Err(AppError::Internal(
                "guest booking has already been promoted".to_string(),
            ));
        }

        // Enforce the payment window (starts at confirmation). If the deadline
        // has passed, mark the booking PAYMENT_EXPIRED and reject the promotion.
        if let Some(payment_deadline) = guest.payment_expires_at {
            if Utc::now().naive_utc() > payment_deadline {
                let expire_update = GuestBookingForUpdateRequest {
                    guest_email: None,
                    guest_name: None,
                    total_amount: None,
                    currency: None,
                    status: Some(GuestBookingStatus::PAYMENT_EXPIRED.to_string()),
                    booking_reference: None,
                    metadata: None,
                    promoted_booking_id: None,
                    confirmed_at: None,
                    payment_expires_at: None,
                };
                let _ = GuestBookingMutation::update_guest_booking_with_txn(
                    guest_booking_id,
                    expire_update.into(),
                    &txn,
                )
                .await;
                // Record the PAYMENT_EXPIRED transition (best-effort, same txn).
                let history = GuestBookingHistoryEntry::new(
                    guest_booking_id,
                    GuestBookingHistoryEvent::PAYMENT_EXPIRED,
                )
                .with_status_change(
                    Some(guest.status.clone()),
                    Some(GuestBookingStatus::PAYMENT_EXPIRED.to_string()),
                )
                .with_actor(Some(user_id))
                .with_note(Some("payment window elapsed".to_string()));
                let _ = Self::append_history(history, &txn).await;
                txn.commit().await.map_err(|_| AppError::Unknown)?;
                return Err(AppError::Internal(
                    "guest booking payment window has elapsed".to_string(),
                ));
            }
        }

        // 2. Load seat items.
        let items = GuestBookingItemQuery::list_by_guest_booking_with_txn(guest_booking_id, &txn)
            .await
            .map_err(|_| AppError::Unknown)?;
        if items.is_empty() {
            return Err(AppError::Internal(
                "guest booking has no seat items".to_string(),
            ));
        }

        // 3. Create the core booking row, carrying over the guest booking's
        //    classification (booking_type / booking_mode) and polymorphic target
        //    (resource_type / resource_id). No longer event-specific.
        let booking_reference = Self::generate_reference("BK");
        let core_dto = BookingForCreateDto {
            booking_type: guest.booking_type.clone(),
            booking_mode: guest.booking_mode.clone(),
            resource_type: guest.resource_type.clone(),
            resource_id: guest.resource_id,
            external_ref: guest.external_ref.clone(),
            user_id,
            total_amount: guest.total_amount,
            currency: guest.currency.clone(),
            status: "CONFIRMED".to_string(),
            payment_status: "SUCCESS".to_string(),
            booking_reference,
            metadata: guest.metadata.clone(),
        };
        let booking_id = BookingMutation::create_booking_with_txn(core_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error creating booking during promotion: {:?}", e);
                AppError::Internal("Failed to create booking".to_string())
            })?;

        // 4. Mode-specific child row. Today promotion supports the CAPACITY
        //    strategy (the default): the container is the polymorphic target
        //    (`resource_id`) and quantity is the number of seat items. Other
        //    modes are carried on the core row but have no child row created
        //    here yet (future work).
        if guest.booking_mode == BookingMode::Capacity.as_str() {
            let container_id = guest.resource_id.ok_or_else(|| {
                AppError::Internal(
                    "CAPACITY guest booking requires a resource_id (container) to promote"
                        .to_string(),
                )
            })?;
            BookingCapacityMutation::create_with_txn(
                BookingCapacityForCreateDto {
                    booking_id,
                    container_id,
                    quantity: items.len() as i32,
                },
                &txn,
            )
            .await
            .map_err(|e| {
                debug!("Error creating booking capacity during promotion: {:?}", e);
                AppError::Internal("Failed to create booking capacity".to_string())
            })?;
        }

        // 5. One booking_item per guest seat item.
        for item in &items {
            BookingItemMutation::create_booking_item_with_txn(
                BookingItemForCreateDto {
                    booking_id,
                    item_type: item.item_type.clone(),
                    item_id: item.item_id,
                    price: item.price,
                    metadata: item.metadata.clone(),
                },
                &txn,
            )
            .await
            .map_err(|e| {
                debug!("Error creating booking item during promotion: {:?}", e);
                AppError::Internal("Failed to create booking item".to_string())
            })?;
        }

        // 6. History entry.
        let history: BookingHistoryEntry =
            BookingHistoryEntry::new(booking_id, BookingHistoryEvent::CREATED)
                .with_status_change(None, Some("CONFIRMED".to_string()))
                .with_actor(Some(user_id))
                .with_note(Some(format!(
                    "promoted from guest booking {}",
                    guest_booking_id
                )));
        BookingHistoryMutation::append_with_txn(history.into(), &txn)
            .await
            .map_err(|e| {
                debug!("Error appending booking history during promotion: {:?}", e);
                AppError::Internal("Failed to append booking history".to_string())
            })?;

        // 7. Mark the guest booking PROMOTED + link the real booking.
        let update = GuestBookingForUpdateRequest {
            guest_email: None,
            guest_name: None,
            total_amount: None,
            currency: None,
            status: Some(GuestBookingStatus::PROMOTED.to_string()),
            booking_reference: None,
            metadata: None,
            promoted_booking_id: Some(booking_id),
            confirmed_at: None,
            payment_expires_at: None,
        };
        GuestBookingMutation::update_guest_booking_with_txn(guest_booking_id, update.into(), &txn)
            .await
            .map_err(|e| {
                debug!("Error marking guest booking promoted: {:?}", e);
                AppError::Internal("Failed to update guest booking".to_string())
            })?;

        // 8. Record the PROMOTED lifecycle event on the guest booking, linking
        //    the newly created real booking id.
        let history =
            GuestBookingHistoryEntry::new(guest_booking_id, GuestBookingHistoryEvent::PROMOTED)
                .with_status_change(
                    Some(guest.status.clone()),
                    Some(GuestBookingStatus::PROMOTED.to_string()),
                )
                .with_actor(Some(user_id))
                .with_note(Some(format!("promoted to booking {}", booking_id)));
        Self::append_history(history, &txn).await?;

        txn.commit().await.map_err(|_| AppError::Unknown)?;
        Ok(booking_id)
    }

    /// Generate a unique, human-readable booking reference like `GBK-1A2B3C4D`.
    fn generate_reference(prefix: &str) -> String {
        let short = Uuid::new_v4().simple().to_string()[..8].to_uppercase();
        format!("{}-{}", prefix, short)
    }

    /// Normalize an origin for comparison: trim, lowercase, and drop any
    /// trailing slash. e.g. `https://Site-A.com/` -> `https://site-a.com`.
    fn normalize_origin(origin: &str) -> String {
        origin.trim().trim_end_matches('/').to_lowercase()
    }

    /// Check a (normalized) origin against the allowlist from the
    /// `GUEST_BOOKING_ALLOWED_ORIGINS` env var (comma-separated). If the var is
    /// unset or empty, no origin is allowed (fail closed).
    fn is_origin_allowed(origin: &str) -> bool {
        let allowed = std::env::var("GUEST_BOOKING_ALLOWED_ORIGINS").unwrap_or_default();
        allowed
            .split(',')
            .map(|s| Self::normalize_origin(s))
            .filter(|s| !s.is_empty())
            .any(|allowed_origin| allowed_origin == origin)
    }

    /// Build the confirmation URL the guest clicks. The booking id and token are
    /// passed as query params; `confirm_path` is normalized to start with `/`.
    fn build_confirm_url(
        site_origin: &str,
        confirm_path: &str,
        guest_booking_id: Uuid,
        confirm_token: &str,
    ) -> String {
        let origin = site_origin.trim_end_matches('/');
        let path = if confirm_path.starts_with('/') {
            confirm_path.to_string()
        } else {
            format!("/{}", confirm_path)
        };
        format!(
            "{}{}?guest_booking_id={}&token={}",
            origin, path, guest_booking_id, confirm_token
        )
    }

    /// Generate a high-entropy (256-bit) confirmation token as a lowercase hex
    /// string, using a cryptographically secure RNG.
    fn generate_token() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// SHA-256 hash (hex) of a token. Only the hash is persisted.
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Constant-time byte comparison to avoid leaking match position via timing.
    fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }
}
