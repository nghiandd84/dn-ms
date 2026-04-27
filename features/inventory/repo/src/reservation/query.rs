use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_inventory_entities::reservation::{ActiveModel, Column, Entity, ModelOptionDto};
use features_inventory_model::reservation::ReservationData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ReservationQueryManager;

pub struct ReservationQuery;

impl ReservationQuery {
    pub async fn get_reservation_by_id(reservation_id: Uuid) -> Result<ReservationData, AppError> {
        let model = ReservationQueryManager::get_by_id_uuid(reservation_id).await?;
        Ok(model.into())
    }

    pub async fn get_reservations<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ReservationData>, AppError> {
        let result = ReservationQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
