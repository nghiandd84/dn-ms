mod change_event_handler;
mod new_event_handler;

use std::collections::HashMap;
use tracing::error;

use change_event_handler::handle_change_event;
use features_inventory_model::state::InventoryAppState;
use new_event_handler::handle_new_event;

use features_event_stream::EventMessage;

use crate::consumers::event_consumer::error::EventError;

pub async fn handle_event_consumer_message(
    message: EventMessage,
    _inventory_state: InventoryAppState,
    _headers: Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = match message {
        EventMessage::New { message: event } => handle_new_event(event).await,
        EventMessage::Update { message: event } => handle_change_event(event).await,
    };

    if let Err(e) = result {
        error!("Error handling event message: {:?}", e);
        return Err(Box::new(EventError::from(e)));
    }
    Ok(())
}
