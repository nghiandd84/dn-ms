use shared_shared_data_core::{
    filter::{convert_filter_param_to_query_string, FilterOperator, FilterParam},
    paging::QueryResult,
};
use shared_shared_macro::RemoteService;

use features_email_template_model::template_translation::TemplateTranslationData;
use shared_shared_middleware::RequestTracingMiddleware;

#[derive(Debug, RemoteService)]
#[remote(name(email_template_service))]
pub struct TemplateTranslationService {}

impl TemplateTranslationService {
    pub async fn get_template_translations(
        template_id: i32,
        language: String,
    ) -> Result<TemplateTranslationData, String> {
        let template_translations_endpoint = std::env::var("TEMPLATE_TRANSLATIONS_ENDPOINT_SEARCH")
            .expect("TEMPLATE_TRANSLATIONS_ENDPOINT_SEARCH must be set");

        let template_id_param = FilterParam {
            name: "template_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(template_id),
            raw_value: template_id.to_string(),
        };
        let template_id_query_param = convert_filter_param_to_query_string(&template_id_param);

        let language_param = FilterParam {
            name: "language_code".to_string(),
            operator: FilterOperator::Equal,
            value: Some(language.clone()),
            raw_value: language,
        };
        let language_query_param = convert_filter_param_to_query_string(&language_param);

        let url = format!(
            "{}?{}&{}",
            template_translations_endpoint, template_id_query_param, language_query_param
        );
        let res = Self::call_api(url, reqwest::Method::GET, None, HashMap::new()).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data: serde_json::Value = res.unwrap();
        let template_translation: QueryResult<TemplateTranslationData> =
            serde_json::from_value(data).map_err(|e| {
                error!("Failed to deserialize template translation data: {}", e);
                e.to_string()
            })?;
        if template_translation.result.is_empty() {
            return Err("Template translation not found".to_string());
        }
        let template_translation_data = template_translation.result.into_iter().next().unwrap();
        Ok(template_translation_data)
    }
}
