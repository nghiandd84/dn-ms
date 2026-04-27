use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_email_template_entities::email_templates::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_email_template_model::email_template::EmailTemplateData;

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
struct EmailTemplateQueryManager;



pub struct EmailTemplateQuery {}

impl EmailTemplateQuery {
    pub async fn get<'a>(id: i32) -> Result<EmailTemplateData, DbErr> {
        let model = EmailTemplateQueryManager::get_by_id_i32(id).await?;
        let user_data: EmailTemplateData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<EmailTemplateData>, DbErr> {
        let result = EmailTemplateQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
