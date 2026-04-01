use axum::http::request::Parts;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_wallet_model::idempotency::{
    IdempotencyKeyData, IdempotencyKeyForCreateRequest, IdempotencyKeyForUpdateRequest,
};
use features_wallet_repo::idempotency::{IdempotencyMutation, IdempotencyQuery};

pub struct IdempotencyService;

impl IdempotencyService {
    const DEFAULT_TTL_HOURS: i64 = 24;

    /// Check if key exists and return cached response or process
    pub async fn check_or_create(key: &str, endpoint: &str) -> Result<IdempotencyState, AppError> {
        match IdempotencyMutation::create_if_not_exists(
            key,
            endpoint,
            "PENDING",
            None,
            Self::DEFAULT_TTL_HOURS,
        )
        .await
        {
            Ok(_) => {
                // Insert succeeded, we own this request
                Ok(IdempotencyState::Process)
            }
            Err(_) => {
                // Key already exists, fetch it
                let existing = IdempotencyQuery::get_idempotency_key_by_key(key).await?;

                match existing.state.as_deref() {
                    Some("PENDING") => {
                        // Another request in progress
                        Err(AppError::DuplicateEntry(
                            "Request already in progress. Please retry.".to_string(),
                        ))
                    }
                    Some("COMPLETED") => {
                        // Return cached response
                        Ok(IdempotencyState::ReturnCached(
                            existing
                                .response_body
                                .and_then(|v| serde_json::to_string(&v).ok())
                                .unwrap_or_default(),
                        ))
                    }
                    _ => Err(AppError::Internal("Unknown idempotency state".to_string())),
                }
            }
        }
    }

    /// Mark request as completed and store response
    pub async fn complete(key: &str, status: u16, response_body: &str) -> Result<(), AppError> {
        let result = IdempotencyMutation::update_completed(key, status as i32, response_body).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                debug!("Error updating idempotency key: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update idempotency key".to_string(),
                ))
            }
        }
    }

    pub async fn create_idempotency_key(
        request: IdempotencyKeyForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let id = IdempotencyMutation::create_idempotency_key(request.into()).await;
        match id {
            Ok(id) => Ok(id),
            Err(e) => {
                debug!("Error creating idempotency key: {:?}", e);
                Err(AppError::Internal(
                    "Failed to create idempotency key".to_string(),
                ))
            }
        }
    }

    pub async fn get_idempotency_key_by_id(id: Uuid) -> Result<IdempotencyKeyData, AppError> {
        IdempotencyQuery::get_idempotency_key_by_id(id).await
    }

    pub async fn get_idempotency_key_by_key(key: &str) -> Result<IdempotencyKeyData, AppError> {
        IdempotencyQuery::get_idempotency_key_by_key(key).await
    }

    pub async fn get_idempotency_keys(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<IdempotencyKeyData>, AppError> {
        IdempotencyQuery::get_idempotency_keys(pagination, order, filters).await
    }

    pub async fn update_idempotency_key(
        id: Uuid,
        request: IdempotencyKeyForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = IdempotencyMutation::update_idempotency_key(id, request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating idempotency key: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update idempotency key".to_string(),
                ))
            }
        }
    }

    pub async fn delete_idempotency_key(id: Uuid) -> Result<bool, AppError> {
        let result = IdempotencyMutation::delete_idempotency_key(id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting idempotency key: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete idempotency key".to_string(),
                ))
            }
        }
    }
}

#[derive(Debug)]
pub enum IdempotencyState {
    Process,
    ReturnCached(String), // cached response body
}
