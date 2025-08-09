use serde_json::de;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_email_template_entities::template_translations::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_email_template_model::template_translation::TemplateTranslationData;

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
struct TemplateTranslationQueryManager;

impl TemplateTranslationQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            debug!("Filter: {:?}", filter_enum);
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct TemplateTranslationQuery {}

impl TemplateTranslationQuery {
    pub async fn get<'a>(db: &'a DbConn, id: i32) -> Result<TemplateTranslationData, DbErr> {
        let model = TemplateTranslationQueryManager::get_by_id_i32(db, id).await?;
        let user_data: TemplateTranslationData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TemplateTranslationData>, DbErr> {
        let result =
            TemplateTranslationQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
