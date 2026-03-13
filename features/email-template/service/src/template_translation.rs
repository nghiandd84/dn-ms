use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
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

    pub async fn get<'a>(id: i32) -> Result<TemplateTranslationData> {
        let email_data = TemplateTranslationQuery::get(id).await?;
        Ok(email_data)
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
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TemplateTranslationData>> {
        let result = TemplateTranslationQuery::search(pagination, order, filters).await?;
        Ok(result)
    }
}
