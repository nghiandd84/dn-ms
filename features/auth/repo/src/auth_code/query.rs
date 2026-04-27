use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
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

pub struct AuthCodeQuery {}

impl AuthCodeQuery {
    pub async fn get<'a>(id: Uuid) -> Result<AuthCodeData, DbErr> {
        let model = AuthCodeQueryManager::get_by_id_uuid(id).await?;
        let user_data: AuthCodeData = model.into();
        Ok(user_data)
    }
    pub async fn get_by_client_id_and_code<'a>(
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
        let filters: FilterCondition = vec![client_id_filter, code_filter].into();
        let query_result = Self::search(&paging, &order, &filters).await?;
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
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<AuthCodeData>, DbErr> {
        let result = AuthCodeQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
