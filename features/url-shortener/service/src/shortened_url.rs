use chrono::Utc;
use rand::Rng;
use tracing::{debug, error};
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_url_shortener_model::{
    cache::CachedUrlData,
    shortened_url::{CreateShortenedUrlRequest, ShortenedUrlData, UpdateShortenedUrlRequest},
};
use features_url_shortener_repo::{ShortenedUrlMutation, ShortenedUrlQuery};

use crate::cache::UrlShortenerCache;

const BASE62_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SHORT_CODE_LENGTH: usize = 7;
const MAX_GENERATION_ATTEMPTS: u8 = 3;

pub struct ShortenedUrlService;

impl ShortenedUrlService {
    /// Generate a random base62 short code.
    fn generate_short_code() -> String {
        let mut rng = rand::thread_rng();
        (0..SHORT_CODE_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..BASE62_CHARSET.len());
                BASE62_CHARSET[idx] as char
            })
            .collect()
    }

    /// Create a new shortened URL.
    /// If `custom_code` is provided, validate uniqueness.
    /// Otherwise, auto-generate a short code with collision retry.
    pub async fn create_short_url(req: CreateShortenedUrlRequest) -> Result<Uuid, AppError> {
        let short_code = if let Some(ref custom_code) = req.custom_code {
            // Validate custom code uniqueness
            match ShortenedUrlQuery::get_by_short_code(custom_code).await {
                Ok(_) => {
                    return Err(AppError::DuplicateEntry(format!(
                        "Short code '{}' is already taken",
                        custom_code
                    )));
                }
                Err(AppError::EntityNotFound { .. }) => custom_code.clone(),
                Err(e) => return Err(e),
            }
        } else {
            // Auto-generate with collision retry
            let mut code;
            let mut attempts = 0;
            loop {
                code = Self::generate_short_code();
                match ShortenedUrlQuery::get_by_short_code(&code).await {
                    Err(AppError::EntityNotFound { .. }) => break,
                    Ok(_) => {
                        attempts += 1;
                        if attempts >= MAX_GENERATION_ATTEMPTS {
                            return Err(AppError::Internal(
                                "Failed to generate unique short code after multiple attempts"
                                    .to_string(),
                            ));
                        }
                        debug!(
                            "Short code collision on '{}', retrying (attempt {})",
                            code, attempts
                        );
                    }
                    Err(e) => return Err(e),
                }
            }
            code
        };

        let dto = req.into_dto(short_code.clone());
        let id = ShortenedUrlMutation::create_shortened_url(dto).await.map_err(|e| {
            error!("Error creating shortened URL: {:?}", e);
            AppError::Internal("Failed to create shortened URL".to_string())
        })?;

        // Cache the new URL
        if let Ok(data) = ShortenedUrlQuery::get_by_id(id, &Default::default()).await {
            let cached = CachedUrlData {
                url_id: id,
                original_url: data.original_url.unwrap_or_default(),
                expires_at: data.expires_at,
                is_active: data.is_active.unwrap_or(true),
            };
            UrlShortenerCache::set_url(&short_code, &cached);
        }

        Ok(id)
    }

    /// Get URL data by short code, using Redis cache.
    /// Returns the cached data or queries DB and caches the result.
    pub async fn get_by_short_code(code: &str) -> Result<CachedUrlData, AppError> {
        // Check cache first
        if let Some(cached) = UrlShortenerCache::get_url(code) {
            return Ok(cached);
        }

        // Cache miss - query DB
        let data = ShortenedUrlQuery::get_by_short_code(code).await?;
        let cached = CachedUrlData {
            url_id: data.id.unwrap_or_default(),
            original_url: data.original_url.unwrap_or_default(),
            expires_at: data.expires_at,
            is_active: data.is_active.unwrap_or(true),
        };

        // Store in cache
        UrlShortenerCache::set_url(code, &cached);

        Ok(cached)
    }

    /// Handle redirect: get URL, validate expiration/active status, record click.
    /// Returns the original URL on success, or an error variant for expired/inactive.
    pub async fn redirect(
        code: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
        referrer: Option<String>,
    ) -> Result<String, AppError> {
        let cached = Self::get_by_short_code(code).await?;

        // Check if active
        if !cached.is_active {
            return Err(AppError::EntityNotFound {
                entity: "This link is inactive".to_string(),
            });
        }

        // Check expiration
        if let Some(expires_at) = cached.expires_at {
            if Utc::now().naive_utc() > expires_at {
                return Err(AppError::EntityNotFound {
                    entity: "This link has expired".to_string(),
                });
            }
        }

        // Fire-and-forget: record click asynchronously
        let url_id = cached.url_id;
        tokio::spawn(async move {
            use features_url_shortener_entities::url_click::UrlClickForCreateDto;
            use features_url_shortener_repo::UrlClickMutation;

            let click_dto = UrlClickForCreateDto {
                url_id,
                ip_address: ip_address.unwrap_or_default(),
                user_agent: user_agent.unwrap_or_default(),
                referrer: referrer.unwrap_or_default(),
                country: String::new(),
            };

            if let Err(e) = UrlClickMutation::record_click(click_dto).await {
                error!("Failed to record click for url_id {}: {:?}", url_id, e);
            }
        });

        Ok(cached.original_url)
    }

    /// Update a shortened URL. Verifies ownership.
    pub async fn update_short_url(
        id: Uuid,
        user_id: Uuid,
        req: UpdateShortenedUrlRequest,
    ) -> Result<bool, AppError> {
        // Verify ownership
        let existing = ShortenedUrlQuery::get_by_id(id, &Default::default()).await?;
        if existing.user_id != Some(user_id) {
            return Err(AppError::Internal("Not authorized to update this URL".to_string()));
        }

        let short_code = existing.short_code.unwrap_or_default();
        let dto: features_url_shortener_entities::shortened_url::ShortenedUrlForUpdateDto =
            req.into();
        let result = ShortenedUrlMutation::update_shortened_url(id, dto).await.map_err(|e| {
            error!("Error updating shortened URL: {:?}", e);
            AppError::Internal("Failed to update shortened URL".to_string())
        })?;

        // Invalidate cache
        UrlShortenerCache::invalidate(&short_code);

        Ok(result)
    }

    /// Delete a shortened URL. Verifies ownership.
    pub async fn delete_short_url(id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        // Verify ownership
        let existing = ShortenedUrlQuery::get_by_id(id, &Default::default()).await?;
        if existing.user_id != Some(user_id) {
            return Err(AppError::Internal("Not authorized to delete this URL".to_string()));
        }

        let short_code = existing.short_code.unwrap_or_default();
        let result = ShortenedUrlMutation::delete_shortened_url(id).await.map_err(|e| {
            error!("Error deleting shortened URL: {:?}", e);
            AppError::Internal("Failed to delete shortened URL".to_string())
        })?;

        // Invalidate cache
        UrlShortenerCache::invalidate(&short_code);

        Ok(result)
    }

    /// List URLs for a specific user with pagination and filters.
    pub async fn list_user_urls(
        user_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ShortenedUrlData>, AppError> {
        ShortenedUrlQuery::get_user_urls(user_id, pagination, order, filters).await
    }
}
