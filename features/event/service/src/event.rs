use features_event_entities::event::Column;
use sea_orm::Iden;
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_event_model::{EventData, EventForCreateRequest, EventForUpdateRequest};
use features_event_repo::event::{EventMutation, EventQuery};
use features_event_stream::{ChangeEventMessage, EventMessage, NewEventMessage};

pub struct EventService {}

impl EventService {
    pub async fn create_event<'a>(
        event_request: EventForCreateRequest,
        producer: &'a Producer,
    ) -> Result<Uuid, AppError> {
        let event_id = EventMutation::create_event(event_request.clone().into()).await;
        let id = match event_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating event: {:?}", e);
                return Err(AppError::Internal("Failed to create event".to_string()));
            }
        };
        let new_event_message = EventMessage::New {
            message: NewEventMessage {
                id: id.clone(),
                total_seats: event_request.total_seats,
            },
        };
        let message = ProducerMessage {
            payload: new_event_message,
            key: None,
        };
        producer.send(&message).await.map_err(|e| {
            debug!("Error sending new event message to Kafka: {:?}", e.reason);
            AppError::Unknown
        })?;
        Ok(id)
    }

    pub async fn get_event_by_id<'a>(event_id: Uuid) -> Result<EventData, AppError> {
        EventQuery::get_event_by_id(event_id).await
    }

    pub async fn get_events_by_status(
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
        EventQuery::get_events(&pagination, &order, &filters).await
    }

    pub async fn get_events<'a>(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<EventData>, AppError> {
        EventQuery::get_events(pagination, order, filters).await
    }

    pub async fn update_event(
        event_id: Uuid,
        event_request: EventForUpdateRequest,
        producer: &Producer,
    ) -> Result<bool, AppError> {
        let result = EventMutation::update_event(event_id, event_request.clone().into()).await;
        match result {
            Ok(success) => {
                if event_request.total_seats.is_some() {
                    let new_event_message = EventMessage::Update {
                        message: ChangeEventMessage {
                            id: event_id,
                            total_seats: event_request.total_seats.unwrap(),
                        },
                    };
                    let message = ProducerMessage {
                        payload: new_event_message,
                        key: None,
                    };
                    producer.send(&message).await.map_err(|e| {
                        debug!("Error sending new event message to Kafka: {:?}", e.reason);
                        AppError::Unknown
                    })?;
                }
                Ok(success)
            }
            Err(e) => {
                debug!("Error updating event: {:?}", e);
                Err(AppError::Internal("Failed to update event".to_string()))
            }
        }
    }

    pub async fn delete_event(event_id: Uuid) -> Result<bool, AppError> {
        let result = EventMutation::delete_event(event_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting event: {:?}", e);
                Err(AppError::Internal("Failed to delete event".to_string()))
            }
        }
    }
}
