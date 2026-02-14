use std::future::Future;
use tracing::debug;

use features_event_stream::ChangeEventMessage;

use crate::consumers::event_consumer::error::EventError;

pub fn handle_change_event(
    message: ChangeEventMessage,
) -> impl Future<Output = Result<(), EventError>> {
    async move {
        debug!("Handling change event: {:?}", message);
        // Process the change event
        Ok(())
    }
}
