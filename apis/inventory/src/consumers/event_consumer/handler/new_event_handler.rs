use std::future::Future;

use features_event_stream::NewEventMessage;

use crate::consumers::event_consumer::error::EventError;

pub fn handle_new_event(message: NewEventMessage) -> impl Future<Output = Result<(), EventError>> {
    async move {
        // Process the new event
        Ok(())
    }
}
