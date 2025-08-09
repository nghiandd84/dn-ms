use sea_orm::DbConn;

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::FilterEnum,
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
    pub async fn create<'a>(
        db: &'a DbConn,
        request: TemplatePlaceholderForCreateRequest,
    ) -> Result<i32> {
        let result = TemplatePlaceholderMutation::create(db, request.into()).await?;
        Ok(result)
    }

    pub async fn get<'a>(db: &'a DbConn, id: i32) -> Result<TemplatePlaceholderData> {
        let email_data = TemplatePlaceholderQuery::get(db, id).await?;
        Ok(email_data)
    }

    pub async fn update<'a>(
        db: &'a DbConn,
        id: i32,
        request: TemplatePlaeholderForUpdateRequest,
    ) -> Result<bool> {
        let result = TemplatePlaceholderMutation::update(db, id, request.into()).await?;
        Ok(result)
    }

    pub async fn delete<'a>(db: &'a DbConn, id: i32) -> Result<bool> {
        let result = TemplatePlaceholderMutation::delete(db, id).await?;
        Ok(result)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TemplatePlaceholderData>> {
        let result = TemplatePlaceholderQuery::search(db, pagination, order, filters).await?;
        Ok(result)
    }
}
