use uuid::Uuid;

use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_url_shortener_entities::url_click::UrlClickForCreateDto;
use features_url_shortener_model::url_click::UrlClickData;
use features_url_shortener_repo::{UrlClickMutation, UrlClickQuery};

pub struct UrlClickService;

impl UrlClickService {
    /// Record a click for a URL.
    pub async fn record_click(
        url_id: Uuid,
        ip_address: String,
        user_agent: String,
        referrer: String,
    ) -> Result<Uuid, AppError> {
        let dto = UrlClickForCreateDto {
            url_id,
            ip_address,
            user_agent,
            referrer,
            country: String::new(),
        };
        let id = UrlClickMutation::record_click(dto).await.map_err(|e| {
            tracing::error!("Error recording click: {:?}", e);
            AppError::Internal("Failed to record click".to_string())
        })?;
        Ok(id)
    }

    /// Get paginated clicks for a specific URL.
    pub async fn get_clicks(
        url_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<UrlClickData>, AppError> {
        UrlClickQuery::get_clicks_by_url_id(url_id, pagination, order).await
    }
}
