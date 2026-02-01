use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_translation_entities::project::{ActiveModel, Column, Entity, ModelOptionDto};
use features_translation_model::ProjectData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ProjectQueryManager;

impl ProjectQueryManager {
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

pub struct ProjectQuery;

impl ProjectQuery {
    pub async fn get_project_by_id(db: &DbConn, project_id: Uuid) -> Result<ProjectData, AppError> {
        let model = ProjectQueryManager::get_by_id_uuid(db, project_id).await?;
        Ok(model.into())
    }

    pub async fn get_projects<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<ProjectData>, AppError> {
        let result = ProjectQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
