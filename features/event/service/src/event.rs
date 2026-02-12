use features_event_entities::event::Column;
use sea_orm::{DbConn, Iden};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_event_model::{EventData, EventForCreateRequest, EventForUpdateRequest};
use features_event_repo::event::{EventMutation, EventQuery};

pub struct EventService {}

impl EventService {
    pub async fn create_event<'a>(
        db: &'a DbConn,
        event_request: EventForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let event_id = EventMutation::create_event(db, event_request.into()).await;
        let id = match event_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating event: {:?}", e);
                return Err(AppError::Internal("Failed to create event".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_event_by_id<'a>(
        db: &'a DbConn,
        event_id: Uuid,
    ) -> Result<EventData, AppError> {
        EventQuery::get_event_by_id(db, event_id).await
    }

    pub async fn get_events_by_status(
        db: &DbConn,
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<EventData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        EventQuery::get_events(db, &pagination, &order, &filters).await
    }

    pub async fn get_events<'a>(
        db: &'a DbConn,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<EventData>, AppError> {
        EventQuery::get_events(db, pagination, order, filters).await
    }

    pub async fn update_event(
        db: &DbConn,
        event_id: Uuid,
        event_request: EventForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = EventMutation::update_event(db, event_id, event_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating event: {:?}", e);
                Err(AppError::Internal("Failed to update event".to_string()))
            }
        }
    }

    pub async fn delete_event(db: &DbConn, event_id: Uuid) -> Result<bool, AppError> {
        let result = EventMutation::delete_event(db, event_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting event: {:?}", e);
                Err(AppError::Internal("Failed to delete event".to_string()))
            }
        }
    }
}
