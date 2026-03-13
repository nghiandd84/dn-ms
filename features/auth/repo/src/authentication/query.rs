use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::authentication::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::authentication::AuthenticationRequestData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct AuthenticationRequestQueryManager;

impl AuthenticationRequestQueryManager {
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

pub struct AuthenticationRequestQuery {}

impl AuthenticationRequestQuery {
    pub async fn get<'a>(id: Uuid) -> Result<AuthenticationRequestData, DbErr> {
        let model = AuthenticationRequestQueryManager::get_by_id_uuid(id).await?;
        let authentication_request_data: AuthenticationRequestData = model.into();
        Ok(authentication_request_data)
    }
}
