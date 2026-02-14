use sea_orm::DbConn;
use std::future::Future;
use tracing::debug;

use features_event_stream::NewEventMessage;
use features_inventory_service::SeatService;

use crate::consumers::event_consumer::error::EventError;

pub fn handle_new_event<'a>(
    message: NewEventMessage,
) -> impl Future<Output = Result<(), EventError>> {
    async move {
        debug!("Handling new event: {:?}", message);
        // Process the new event
        Ok(())
    }
}
