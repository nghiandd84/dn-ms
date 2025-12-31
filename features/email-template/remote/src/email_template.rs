use shared_shared_data_core::paging::QueryResult;
use shared_shared_macro::RemoteService;

use features_email_template_model::email_template::EmailTemplateData;
use shared_shared_middleware::RequestTracingMiddleware;

#[derive(Debug, RemoteService)]
#[remote(name(email_template_service))]
pub struct EmailTemplateService {}

impl EmailTemplateService {
    pub async fn get_email_template_by_key(key: String) -> Result<EmailTemplateData, String> {
        let email_template_endpoint = std::env::var("EMAIL_TEMPLATE_ENDPOINT_SEARCH")
            .expect("EMAIL_TEMPLATE_ENDPOINT_SEARCH must be set");

        let url = format!("{}?key=eq|{}", email_template_endpoint, key);

        let res = Self::call_api(
            url,
            reqwest::Method::GET,
            None,
            HashMap::new(),
        )
        .await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data: serde_json::Value = res.unwrap();
        let email_template: QueryResult<EmailTemplateData> =
            serde_json::from_value(data).map_err(|e| {
                error!("Failed to deserialize email template data: {}", e);
                e.to_string()
            })?;
        if email_template.result.is_empty() {
            return Err("Email template not found".to_string());
        }
        let email_template = email_template.result.into_iter().next().unwrap();
        Ok(email_template)
    }
}
