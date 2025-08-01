use uuid::Uuid;

use shared_shared_data_core::{
    filter::{self, FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::auth_code::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::auth_code::AuthCodeData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct AuthCodeQueryManager;

impl AuthCodeQueryManager {
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

pub struct AuthCodeQuery {}

impl AuthCodeQuery {
    pub async fn get<'a>(db: &'a DbConn, id: Uuid) -> Result<AuthCodeData, DbErr> {
        let model = AuthCodeQueryManager::get_by_id_uuid(db, id).await?;
        let user_data: AuthCodeData = model.into();
        Ok(user_data)
    }
    pub async fn get_by_client_id_and_code<'a>(

        db: &'a DbConn,
        client_id: Uuid,
        code: String,
    ) -> Result<AuthCodeData, DbErr> {
        let paging = Pagination::default();
        let order = Order::default();
        let client_id_filter = FilterEnum::Uuid(FilterParam {
            name: Column::ClientId.to_string(),
            value: Some(client_id),
            raw_value: client_id.to_string(),
            operator: FilterOperator::Equal,
        });
        let code_filter = FilterEnum::String(FilterParam {
            name: Column::Code.to_string(),
            value: Some(code.clone()),
            raw_value: code.to_string(),
            operator: FilterOperator::Equal,
        });
        let filters: Vec<FilterEnum> = vec![client_id_filter, code_filter];
        let query_result = Self::search(db, &paging, &order, &filters).await?;
        if query_result.result.is_empty() {
            return Err(DbErr::RecordNotFound(format!(
                "AuthCode with client_id {} not found",
                client_id
            )));
        }
        let user_data: AuthCodeData = query_result.result.into_iter().next().unwrap();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<AuthCodeData>, DbErr> {
        let result = AuthCodeQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
