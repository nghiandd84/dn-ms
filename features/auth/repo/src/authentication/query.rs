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

pub struct AuthenticationRequestQuery {}

impl AuthenticationRequestQuery {
    pub async fn get<'a>(id: Uuid) -> Result<AuthenticationRequestData, DbErr> {
        let model = AuthenticationRequestQueryManager::get_by_id_uuid(id).await?;
        let authentication_request_data: AuthenticationRequestData = model.into();
        Ok(authentication_request_data)
    }
}
