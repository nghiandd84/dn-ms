use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_event_entities::event::{ActiveModel, Column, Entity, ModelOptionDto};
use features_event_model::EventData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct EventQueryManager;



pub struct EventQuery;

impl EventQuery {
    pub async fn get_event_by_id(event_id: Uuid) -> Result<EventData, AppError> {
        let model = EventQueryManager::get_by_id_uuid(event_id).await?;
        Ok(model.into())
    }

    pub async fn get_events<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<EventData>, AppError> {
        let result = EventQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    // pub async fn get_events_by_status(
    //     db: &DbConn,
    //     status: &str,
    // ) -> Result<Vec<EventData>, AppError> {
    //     Entity::find()
    //         .filter(Column::Status.eq(status))
    //         .order_by_asc(Column::EventDate)
    //         .all(db)
    //         .await
    //         .map_err(|_| AppError::Internal("Failed to fetch events".to_string()))?
    //         .into_iter()
    //         .map(|model| EventData {
    //             id: model.id,
    //             event_name: model.event_name,
    //             event_date: model.event_date,
    //             venue_name: model.venue_name,
    //             total_seats: model.total_seats,
    //             status: model.status,
    //             sale_start_time: model.sale_start_time,
    //             created_at: model.created_at,
    //         })
    //         .collect::<Vec<_>>()
    //         .into()
    // }

    // pub async fn get_all_events(db: &DbConn) -> Result<Vec<EventData>, AppError> {
    //     Entity::find()
    //         .order_by_asc(Column::EventDate)
    //         .all(db)
    //         .await
    //         .map_err(|_| AppError::Internal("Failed to fetch events".to_string()))?
    //         .into_iter()
    //         .map(|model| EventData {
    //             id: model.id,
    //             event_name: model.event_name,
    //             event_date: model.event_date,
    //             venue_name: model.venue_name,
    //             total_seats: model.total_seats,
    //             status: model.status,
    //             sale_start_time: model.sale_start_time,
    //             created_at: model.created_at,
    //         })
    //         .collect::<Vec<_>>()
    //         .into()
    // }
}
