use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};

use features_email_template_model::template_translation::{
    TemplateTranslationData, TemplateTranslationForCreateRequest,
    TemplateTranslationForUpdateRequest,
};
use features_email_template_repo::template_translation::{
    TemplateTranslationMutation, TemplateTranslationQuery,
};

pub struct TemplateTranslationService {}

impl TemplateTranslationService {
    pub async fn create<'a>(request: TemplateTranslationForCreateRequest) -> Result<i32> {
        let result = TemplateTranslationMutation::create(request.into()).await?;
        Ok(result)
    }

    pub async fn get<'a>(id: i32, query_params: &QueryParams) -> Result<TemplateTranslationData> {
        let data = TemplateTranslationQuery::get_with_related(id, query_params).await?;
        Ok(data)
    }

    pub async fn update<'a>(id: i32, request: TemplateTranslationForUpdateRequest) -> Result<bool> {
        let result = TemplateTranslationMutation::update(id, request.into()).await?;
        Ok(result)
    }

    pub async fn delete<'a>(id: i32) -> Result<bool> {
        let result = TemplateTranslationMutation::delete(id).await?;
        Ok(result)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
    ) -> Result<QueryResult<TemplateTranslationData>> {
        let result =
            TemplateTranslationQuery::search(pagination, order, filters, query_params).await?;
        Ok(result)
    }
}
