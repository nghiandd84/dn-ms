use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    paging::QueryResult,
};
use shared_shared_macro::RemoteService;

use features_email_template_model::email_template::EmailTemplateData;

#[derive(Debug, RemoteService)]
#[remote(name(email_template_service))]
pub struct EmailTemplateService {}

impl EmailTemplateService {
    pub async fn get_email_template_by_key(key: String) -> Result<EmailTemplateData, String> {
        let email_template_endpoint = std::env::var("EMAIL_TEMPLATE_ENDPOINT_SEARCH")
            .expect("EMAIL_TEMPLATE_ENDPOINT_SEARCH must be set");

        let condition = FilterCondition::leaf(FilterEnum::String(FilterParam {
            name: "key".to_string(),
            operator: FilterOperator::Equal,
            value: Some(key.clone()),
            raw_value: key,
        }));

        let url = format!(
            "{}?{}",
            email_template_endpoint,
            condition.to_query_string()
        );

        let data = Self::call_api(url, reqwest::Method::GET, None, HashMap::new())
            .await
            .map_err(|e| e)?;
        let email_template = QueryResult::<EmailTemplateData>::from_value(data)?;
        if email_template.result.is_empty() {
            return Err("Email template not found".to_string());
        }
        let email_template = email_template.result.into_iter().next().unwrap();
        Ok(email_template)
    }
}
