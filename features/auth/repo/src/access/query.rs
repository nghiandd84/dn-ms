use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::access::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::access::AccessData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct AccessQueryManager;



pub struct AccessQuery {}

impl AccessQuery {
    pub async fn get<'a>(id: Uuid) -> Result<AccessData, DbErr> {
        let model = AccessQueryManager::get_by_id_uuid(id).await?;
        let user_data: AccessData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<AccessData>, DbErr> {
        debug!("AccessQuery::search filters: {:?}", filters);
        let result = AccessQueryManager::filter(pagination, order, filters).await;
        let result = match result {
            Ok(res) => res,
            Err(e) => {
                debug!("AccessQuery::search error: {:?}", e);
                return Err(e);
            }
        };
        debug!("AccessQuery::search result: {:?}", result);
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        debug!("AccessQuery::search mapped_result: {:?}", mapped_result);
        Ok(mapped_result)
    }
}
