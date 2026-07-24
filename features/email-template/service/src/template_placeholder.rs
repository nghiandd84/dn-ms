use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
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

    pub async fn get<'a>(id: i32, query_params: &QueryParams) -> Result<TemplatePlaceholderData> {
        let data = TemplatePlaceholderQuery::get_with_related(id, query_params).await?;
        Ok(data)
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
        query_params: &QueryParams,
    ) -> Result<QueryResult<TemplatePlaceholderData>> {
        let result =
            TemplatePlaceholderQuery::search(pagination, order, filters, query_params).await?;
        Ok(result)
    }
}
