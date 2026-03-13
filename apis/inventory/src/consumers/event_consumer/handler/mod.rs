mod change_event_handler;
mod new_event_handler;

use std::collections::HashMap;
use tracing::{debug, error};

use shared_shared_app::state::AppState;

use features_inventory_model::state::{InventoryAppState, InventoryCacheState};

use change_event_handler::handle_change_event;
use new_event_handler::handle_new_event;

use features_event_stream::EventMessage;

use crate::consumers::event_consumer::error::EventError;

pub async fn handle_event_consumer_message(
    message: EventMessage,
    state: AppState<InventoryAppState, InventoryCacheState>,
    // conn: DatabaseConnection,
    // _inventory_state: InventoryAppState,
    _headers: Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Received event message: {:?}", message);
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
