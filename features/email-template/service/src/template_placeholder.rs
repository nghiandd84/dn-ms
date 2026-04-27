use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
};

use features_email_template_model::template_placeholder::{
    TemplatePlaceholderData, TemplatePlaceholderForCreateRequest,
    TemplatePlaeholderForUpdateRequest,
};
use features_email_template_repo::template_placeholder::{
    TemplatePlaceholderMutation, TemplatePlaceholderQuery,
};

pub struct TemplatePlaceholderService {}

impl TemplatePlaceholderService {
    pub async fn create<'a>(request: TemplatePlaceholderForCreateRequest) -> Result<i32> {
        let result = TemplatePlaceholderMutation::create(request.into()).await?;
        Ok(result)
    }

    pub async fn get<'a>(id: i32) -> Result<TemplatePlaceholderData> {
        let email_data = TemplatePlaceholderQuery::get(id).await?;
        Ok(email_data)
    }

    pub async fn update<'a>(id: i32, request: TemplatePlaeholderForUpdateRequest) -> Result<bool> {
        let result = TemplatePlaceholderMutation::update(id, request.into()).await?;
        Ok(result)
    }

    pub async fn delete<'a>(id: i32) -> Result<bool> {
        let result = TemplatePlaceholderMutation::delete(id).await?;
        Ok(result)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<TemplatePlaceholderData>> {
        let result = TemplatePlaceholderQuery::search(pagination, order, filters).await?;
        Ok(result)
    }
}
