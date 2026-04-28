use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    paging::QueryResult,
};
use shared_shared_macro::RemoteService;

use features_email_template_model::template_translation::TemplateTranslationData;

#[derive(Debug, RemoteService)]
#[remote(name(email_template_service))]
pub struct TemplateTranslationService {}

impl TemplateTranslationService {
    pub async fn get_template_translations(
        template_id: i32,
        language: String,
    ) -> Result<TemplateTranslationData, String> {
        let endpoint = std::env::var("TEMPLATE_TRANSLATIONS_ENDPOINT_SEARCH")
            .expect("TEMPLATE_TRANSLATIONS_ENDPOINT_SEARCH must be set");

        let condition = FilterCondition::And(vec![
            FilterCondition::Leaf(FilterEnum::I32(FilterParam {
                name: "template_id".to_string(),
                operator: FilterOperator::Equal,
                value: Some(template_id),
                raw_value: template_id.to_string(),
            })),
            FilterCondition::Leaf(FilterEnum::String(FilterParam {
                name: "language_code".to_string(),
                operator: FilterOperator::Equal,
                value: Some(language.clone()),
                raw_value: language,
            })),
        ]);

        let url = format!("{}?{}", endpoint, condition.to_query_string());
        let data = Self::call_api(url, reqwest::Method::GET, None, HashMap::new())
            .await
            .map_err(|e| e)?;
        let result = QueryResult::<TemplateTranslationData>::from_value(data)?;
        if result.result.is_empty() {
            return Err("Template translation not found".to_string());
        }
        Ok(result.result.into_iter().next().unwrap())
    }
}
