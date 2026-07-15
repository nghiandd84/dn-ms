use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_macro::Response;

use features_url_shortener_entities::url_click::ModelOptionDto;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response)]
pub struct UrlClickData {
    pub id: Option<Uuid>,
    pub url_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub country: Option<String>,
    pub clicked_at: Option<DateTime>,
}

impl From<ModelOptionDto> for UrlClickData {
    fn from(val: ModelOptionDto) -> Self {
        UrlClickData {
            id: val.id,
            url_id: val.url_id,
            ip_address: val.ip_address,
            user_agent: val.user_agent,
            referrer: val.referrer,
            country: val.country,
            clicked_at: val.clicked_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
pub struct ClickStats {
    pub total_clicks: i64,
    pub clicks_by_day: Vec<DayClickCount>,
    pub top_referrers: Vec<ReferrerCount>,
    pub top_countries: Vec<CountryCount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
pub struct DayClickCount {
    pub date: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
pub struct ReferrerCount {
    pub referrer: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
pub struct CountryCount {
    pub country: String,
    pub count: i64,
}
