use rand::Rng;
use sha2::{Digest, Sha256};
use tracing::error;
use uuid::Uuid;

use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_url_shortener_entities::api_key::ApiKeyForCreateDto;
use features_url_shortener_model::api_key::{ApiKeyCreatedResponse, ApiKeyData, CreateApiKeyRequest};
use features_url_shortener_repo::{ApiKeyMutation, ApiKeyQuery};

pub struct ApiKeyService;

impl ApiKeyService {
    /// Generate a random 32-byte API key and return its hex representation.
    fn generate_api_key() -> String {
        let mut rng = rand::thread_rng();
        let key_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        key_bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Hash a plaintext API key with SHA-256.
    fn hash_key(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create a new API key for a user.
    /// Returns the plaintext key (shown only once) along with metadata.
    pub async fn create_api_key(
        user_id: Uuid,
        req: CreateApiKeyRequest,
    ) -> Result<ApiKeyCreatedResponse, AppError> {
        let plaintext_key = Self::generate_api_key();
        let key_hash = Self::hash_key(&plaintext_key);

        let dto = ApiKeyForCreateDto {
            user_id,
            key_hash,
            name: req.name.clone(),
        };

        let id = ApiKeyMutation::create_api_key(dto).await.map_err(|e| {
            error!("Error creating API key: {:?}", e);
            AppError::Internal("Failed to create API key".to_string())
        })?;

        Ok(ApiKeyCreatedResponse {
            id,
            name: req.name,
            key: plaintext_key,
            created_at: chrono::Utc::now().naive_utc(),
        })
    }

    /// Validate an API key by hashing and looking up in DB.
    pub async fn validate_api_key(key: &str) -> Result<ApiKeyData, AppError> {
        let key_hash = Self::hash_key(key);
        let api_key = ApiKeyQuery::get_by_key_hash(&key_hash).await?;

        if api_key.is_active != Some(true) {
            return Err(AppError::Internal("API key is revoked".to_string()));
        }

        Ok(api_key)
    }

    /// Revoke an API key. Verifies ownership.
    pub async fn revoke_api_key(id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        // Get key to verify ownership
        let keys = ApiKeyQuery::list_by_user_id(&user_id, &Pagination::default(), &Order::default())
            .await?;
        let key_exists = keys.result.iter().any(|k| k.id == Some(id));
        if !key_exists {
            return Err(AppError::Internal(
                "Not authorized to revoke this API key".to_string(),
            ));
        }

        ApiKeyMutation::delete_api_key(id).await.map_err(|e| {
            error!("Error revoking API key: {:?}", e);
            AppError::Internal("Failed to revoke API key".to_string())
        })
    }

    /// List API keys for a user (without exposing hashes).
    pub async fn list_user_keys(
        user_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<ApiKeyData>, AppError> {
        ApiKeyQuery::list_by_user_id(user_id, pagination, order).await
    }
}
