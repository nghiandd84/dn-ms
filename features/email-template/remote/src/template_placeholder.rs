use shared_shared_data_core::{
    filter::{convert_filter_param_to_query_string, FilterOperator, FilterParam},
    paging::QueryResult,
};
use shared_shared_macro::RemoteService;

use features_email_template_model::template_placeholder::TemplatePlaceholderData;

#[derive(Debug, RemoteService)]
#[remote(name(email_template_service))]
pub struct TemplatePlaceholderService {}

impl TemplatePlaceholderService {
    pub async fn get_template_holder_by_template_id(
        template_id: i32,
    ) -> Result<Vec<TemplatePlaceholderData>, String> {
        let email_template_endpoint = std::env::var("TEMPLATE_PLACEHOLDER_ENDPOINT_SEARCH")
            .expect("TEMPLATE_PLACEHOLDER_ENDPOINT_SEARCH must be set");

        let template_id_param = FilterParam {
            name: "template_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(template_id),
            raw_value: template_id.to_string(),
        };
        let template_id_query_param = convert_filter_param_to_query_string(&template_id_param);

        let url = format!("{}?{}", email_template_endpoint, template_id_query_param);

        let res = Self::call_api(url, reqwest::Method::GET, None, HashMap::new()).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data: serde_json::Value = res.unwrap();
        let result: QueryResult<TemplatePlaceholderData> =
            serde_json::from_value(data).map_err(|e| {
                error!("Failed to deserialize email template data: {}", e);
                e.to_string()
            })?;
        if result.result.is_empty() {
            return Err("Email template not found".to_string());
        }
        Ok(result.result)
    }
}
