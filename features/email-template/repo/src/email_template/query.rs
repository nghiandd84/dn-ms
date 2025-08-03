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

impl EmailTemplateQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct EmailTemplateQuery {}

impl EmailTemplateQuery {
    pub async fn get<'a>(db: &'a DbConn, id: i32) -> Result<EmailTemplateData, DbErr> {
        let model = EmailTemplateQueryManager::get_by_id_i32(db, id).await?;
        let user_data: EmailTemplateData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<EmailTemplateData>, DbErr> {
        let result = EmailTemplateQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
