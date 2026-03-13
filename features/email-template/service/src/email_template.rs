use tracing::debug;

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};

use features_email_template_model::email_template::{
    EmailTemplateData, EmailTemplateForCreateRequest, EmailTemplateForUpdateRequest,
};
use features_email_template_repo::email_template::{EmailTemplateMutation, EmailTemplateQuery};

pub struct EmailTemplateService {}

impl EmailTemplateService {
    pub async fn create<'a>(request: EmailTemplateForCreateRequest) -> Result<i32> {
        let result = EmailTemplateMutation::create(request.into()).await?;

        debug!("Email template was created with ID: {}", result);
        Ok(result)
    }

    pub async fn get<'a>(id: i32) -> Result<EmailTemplateData> {
        let email_data = EmailTemplateQuery::get(id).await?;
        Ok(email_data)
    }

    pub async fn update<'a>(id: i32, request: EmailTemplateForUpdateRequest) -> Result<bool> {
        let result = EmailTemplateMutation::update(id, request.into()).await?;

        debug!("Email template was updated with ID: {}", result);
        Ok(result)
    }

    pub async fn delete<'a>(id: i32) -> Result<bool> {
        let result = EmailTemplateMutation::delete(id).await?;
        Ok(result)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<EmailTemplateData>> {
        let result = EmailTemplateQuery::search(pagination, order, filters).await?;
        Ok(result)
    }
}
