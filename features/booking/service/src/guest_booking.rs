use chrono::Utc;
use rand::RngCore;
use sea_orm::TransactionTrait;
use sha2::{Digest, Sha256};
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_config::db::DB_WRITE;
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_booking_model::booking::BookingMode;
use features_booking_model::booking_history::{BookingHistoryEntry, BookingHistoryEvent};
use features_booking_model::guest_booking::{
    GuestBookingConfirmRequest, GuestBookingData, GuestBookingForCreateRequest,
    GuestBookingForUpdateRequest, GuestBookingStatus,
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
use features_booking_repo::guest_booking_item::{
    GuestBookingItemMutation, GuestBookingItemQuery,
};

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
            debug!("Rejected guest booking: site_origin not allowed: {}", site_origin);
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
        let guest_booking_id =
            GuestBookingMutation::create_guest_booking_with_txn(core_dto, &txn)
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
                event_id: request.event_id,
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
            return Err(AppError::Internal(
                "guest booking is cancelled".to_string(),
            ));
        }
        if existing.status == GuestBookingStatus::EXPIRED {
            return Err(AppError::Internal(
                "guest booking has expired".to_string(),
            ));
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

        txn.commit().await.map_err(|_| AppError::Unknown)?;
        Ok(ok)
    }

    /// Promote a CONFIRMED guest booking into a real booking.
    ///
    /// In a single transaction: creates a core `bookings` row (EVENT / CAPACITY),
    /// its capacity child row, one `booking_item` per guest seat item, a
    /// `booking_history` CREATED entry, then marks the guest booking PROMOTED and
    /// records the new booking id. Returns the new booking id.
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

        // 3. Create the core booking row (EVENT / CAPACITY).
        let booking_reference = Self::generate_reference("BK");
        let core_dto = BookingForCreateDto {
            booking_type: "EVENT".to_string(),
            booking_mode: BookingMode::Capacity.as_str().to_string(),
            resource_type: Some("event".to_string()),
            resource_id: Some(guest.event_id),
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

        // 4. CAPACITY child row (container = event, quantity = number of seats).
        BookingCapacityMutation::create_with_txn(
            BookingCapacityForCreateDto {
                booking_id,
                container_id: guest.event_id,
                quantity: items.len() as i32,
            },
            &txn,
        )
        .await
        .map_err(|e| {
            debug!("Error creating booking capacity during promotion: {:?}", e);
            AppError::Internal("Failed to create booking capacity".to_string())
        })?;

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
        let history: BookingHistoryEntry = BookingHistoryEntry::new(booking_id, BookingHistoryEvent::CREATED)
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
