use tracing::debug;

use features_event_stream::NewEventMessage;

use crate::consumers::event_consumer::error::EventError;

pub async fn handle_new_event<'a>(message: NewEventMessage) -> Result<(), EventError> {
    debug!("Handling new event: {:?}", message);
    // Process the new event - placeholder for payment-specific logic
    Ok(())
}
