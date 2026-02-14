use sea_orm::DbConn;
use tracing::debug;

use features_event_stream::NewEventMessage;
use features_inventory_model::seat::SeatForCreateRequest;
use features_inventory_service::SeatService;

use crate::consumers::event_consumer::error::EventError;

pub async fn handle_new_event<'a>(
    message: NewEventMessage,
    db: &'a DbConn,
) -> Result<(), EventError> {
    // async move {
    debug!("Handling new event: {:?}", message);
    let event_id = message.id;
    let total_seats = message.total_seats;
    let seat_requests = (0..total_seats)
        .enumerate()
        .map(|(index, _)| SeatForCreateRequest {
            event_id,
            seat_number: format!("SEAT_{}", index + 1),
            section: None,
            row_number: None,
            seat_type: None,
            price: 0.0, // Set default price or calculate based on your logic
        })
        .collect::<Vec<_>>();

    let result = SeatService::bulk_create_seats(db, seat_requests).await;
    match result {
        Ok(seat_ids) => {
            debug!("Created seats with IDs: {:?}", seat_ids);
        }
        Err(e) => {
            debug!("Error creating seats: {:?}", e);
            return Err(EventError::FailedToCreateSeats);
        }
    }

    Ok(())
    // }
}
